# -*- coding: utf-8 -*-
import random
import os
from pathlib import Path
import torch
import fm
import numpy as np

from .model import FullModel

torch.set_grad_enabled(False)

project_dir = Path(__file__).parents[1]


# GRAPE generation method
def greedy_decode_guidance(model, input_src, max_len, start_symbol, is_noise):
    if is_noise:
        noise = gaussian_noise(input_src.size(), mean=0.0, std=0.1)
        input_src += noise
    memory = model.adapter(input_src)

    ys = torch.ones(1, 1).fill_(start_symbol).type_as(input_src.data).long()
    for i in range(max_len):
        out = model.decoder(ys, memory)
        selected_tensor = out[0]
        prob = model.generator(selected_tensor[:, -1])
        _, next_word = torch.max(prob, dim=1)
        next_word = next_word.item()
        ys = torch.cat([ys, torch.ones(1, 1).type_as(input_src.data).fill_(next_word)], dim=1).long()

    return ys


def gaussian_noise(size, mean=0.0, std=1.0):
    noise = torch.randn(size) * std + mean
    return noise


def standardization(data):
    mu = torch.mean(data, dim=-1, keepdim=True)
    sigma = torch.std(data, dim=-1, keepdim=True)
    return (data - mu) / sigma


def rna_to_numbers(seq):
    base_to_num = {"A": 1, "C": 2, "G": 3, "U": 4}

    numbers = [base_to_num.get(base, -1) for base in seq.upper()]

    return numbers


def rna_to_onehot(seq):
    mapping = {"A": 0, "C": 1, "G": 2, "U": 3}
    onehot = np.zeros((len(seq), 4), dtype=np.float32)
    for i, base in enumerate(seq.upper()):
        if base in mapping:
            onehot[i, mapping[base]] = 1.0
    onehot = onehot.reshape(-1)
    return onehot


def run_rna_fm(rna_fm_inputs, device, batch_size=100):
    rna_fm_model_path = os.path.join(project_dir, "model_parameters", "RNA-FM_pretrained.pth")
    if os.path.exists(rna_fm_model_path):
        rna_fm_model, alphabet = fm.pretrained.rna_fm_t12(model_location=rna_fm_model_path)
    else:
        rna_fm_model, alphabet = fm.pretrained.rna_fm_t12()
    batch_converter = alphabet.get_batch_converter()
    rna_fm_model.to(device)
    rna_fm_model.eval()

    all_reps = []
    for i in range(0, len(rna_fm_inputs), batch_size):
        batch_inputs = rna_fm_inputs[i : i + batch_size]
        batch_labels, batch_strs, batch_tokens = batch_converter(batch_inputs)
        batch_tokens = batch_tokens.to(device)

        with torch.no_grad():
            results = rna_fm_model(batch_tokens, repr_layers=[12])

        reps = results["representations"][12]
        reps = reps[:, 1:-1, :].detach().cpu().float()
        reps = reps.reshape(reps.shape[0], -1)
        all_reps.append(reps)

    del rna_fm_model
    # torch.cuda.empty_cache()

    return torch.cat(all_reps, dim=0)


def get_samples(seed_seqs, gen_num, device):
    seqs1 = []
    seqs2 = []
    lines = seed_seqs.strip().splitlines()

    for _ in range(gen_num):
        i = random.randint(0, len(lines) - 1)
        j = random.randint(0, len(lines) - 1)
        seqs1.append(lines[i].split()[-1])
        seqs2.append(lines[j].split()[-1])

    reps = run_rna_fm([(i, seq) for i, seq in enumerate(seqs1 + seqs2)], device)
    reps = standardization(reps)
    reps = (reps[:gen_num] + reps[gen_num:]) / 2

    return reps


def generate(
    params,
):
    target = params["target"].lower()
    llm = params["model"].lower()
    seed_seqs = params["seed_seqs"].upper()
    output_file = params["output_file"]
    gen_num = int(params["gen_num"])

    device = torch.device("cpu")
    model_name = f"{target}_{llm}"
    model = FullModel(
        input_dim=12800,
        model_dim=128,
        tgt_size=5,
        n_declayers=2,
        d_ff=128,
        d_k_v=64,
        n_heads=2,
        dropout=0.05,
    )
    model_path = os.path.join(project_dir, "model_parameters", f"{model_name}.model")
    model.load_state_dict(torch.load(model_path, map_location=device))
    model.to(device)
    model.eval()

    samples = get_samples(seed_seqs, gen_num, device)

    generated_seqs = []

    for sample in samples:
        sample = sample.unsqueeze(0).to(device)
        generated_seq = greedy_decode_guidance(model, sample, 20, 0, is_noise=True)
        generated_seq = generated_seq.squeeze().tolist()

        id_to_base = {1: "A", 2: "C", 3: "G", 4: "U"}

        rna_sequence = "".join([id_to_base.get(i, "") for i in generated_seq])
        generated_seqs.append(rna_sequence)

    with open(output_file, "w", buffering=1) as f:
        for seq in generated_seqs:
            f.write(seq + "\n")
