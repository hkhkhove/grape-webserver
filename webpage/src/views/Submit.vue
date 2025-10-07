<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { nanoid } from 'nanoid'

const router = useRouter()

const seedSequence = ref('')
const target = ref('RBD')
const model = ref('RNA-FM')
const count = ref(100)
const isLoading = ref(false)
const submissionError = ref(null)

const defaultState = {
  seedSequence: '',
  target: 'RBD',
  model: 'RNA-FM',
  count: 100,
}

const targetOptions = ['RBD', 'CD3e', 'C-MYC']
const modelOptions = ['RNA-FM']

const MAX_SEED_SEQUENCES = 10000

const validationError = computed(() => {
  if (!seedSequence.value.trim()) return '' //还没有输入的时候，不显示错误（显示空字符串）

  const lines = seedSequence.value.split('\n').filter((line) => line.trim() !== '')

  if (lines.length > MAX_SEED_SEQUENCES) {
    return `Maximum ${MAX_SEED_SEQUENCES} sequences allowed. Current: ${lines.length}`
  }
  for (const [i, line] of lines.entries()) {
    if (!/^[ACGU]+$/i.test(line)) {
      return `Error on line ${i + 1}: Sequence can only contain A, C, G, U characters.`
    }
    if (line.length !== 20) {
      return `Error on line ${i + 1}: Sequence must be 20 characters long.`
    }
  }
  return null // 验证通过
})

// 填充示例数据
function handleExample() {
  seedSequence.value = `UACUCAUGAGCAUGAGUACC
UCGAUGGCGCGCUGUCGCUC
CCGUCGAUACCGACGGCCAG
CUCCCCGGCGGCGGGGAGCC
AGGCGAUCAGGCGCCCAACG
ACUCGGUCGACCGGGCUACA
AUCCUACGGCUGGGCUCUUU
CGUCUGUGACUUGGUGUUCC
AAGUAAUGUUACUCCGCGUU
GACCCUAGCGUGUCACUCGU
UAGUAGUAAUGCUGGCAGCA
UGCAUCGGAAUGCUGGACGA
CGCCACAAGAUCGUGACUGA
CCUUCGCUACAGUGGUUUUG
UCGCUGUAGACAGCGUCAGC
AAGUCGAAAUCAAUUUCGUU
CACCGCACCAACGCGCCCGU
AAUCGAUCAAUGGCGUGACU
ACGGCCUCACACGGUCUACG
CGGCUGCGUAAGCUGCUCAU
GCUGGGACUCACAAAAAUCU
GUGUCGCUAGUCUAUGUGUU
GCUCUGUGUUGACGAACAUC
CCAACAUGUGCCAAGCAUGU
CAUGUAGCAAGCCUCUACAU
UCCUCUUCCAUCCAGCGAGG
GCGUGGCGUCACCUGCUAUC
UACGAGGGGUGACCUUUCAG
UCCUGUUAACCGUAACGAGA
UUCUGCCUCCACCGGCUGUC
GUGUGCGUAGAGUCACGCAG
CCAGCUUCAUCCUAAGUCGU
GAUACCGUGCUGACGCUAGC
CAUCGCUGUAGCUAGCGGAC
UGACGAUUGCAACUCGAUAG
GCUUCGGCCUCUAGCCAAAC
CGACCAAAUGGCUUGACCGG
GGCUCACGAUUCUGCGUCAU
CGUCGGAUGGUCCGGCAUGG
UUGUGCCGCAGGCAUGUAUA
UGACCUUCGGGCCGGAGGUG
ACGAGCUUGGUGUUAAGUAG
CUCGAUAUAUAAGCAUCGCU
AAUCUGGUUAAUGUAUCGGG
UGCACCCGUCGCUGGAUACU
GCCGGCCCUACCUAGUCGAA
AGCACGGCCGCAGUCGUGCA
GAUCUGAUCUAACGGCUCCU
AAUGUCAAUUUGGCGGCGUU
AUAUCUAAUCCGAAGUCAGC`
}

// 重置表单
function handleReset() {
  seedSequence.value = defaultState.seedSequence
  target.value = defaultState.target
  model.value = defaultState.model
  count.value = defaultState.count
  submissionError.value = null
}

// 提交表单到后端
async function handleSubmit() {
  if (validationError.value) return // 如果验证失败，则不提交

  isLoading.value = true
  submissionError.value = null

  const formData = new FormData()
  const taskId = nanoid()

  formData.append('task_id', taskId)
  formData.append('task_name', 'GRAPE')
  formData.append('seed_seqs', seedSequence.value)
  formData.append('target', target.value)
  formData.append('model', model.value)
  formData.append('gen_num', count.value)

  try {
    const response = await fetch('/api/tasks', {
      method: 'POST',
      body: formData,
    })

    if (!response.ok) {
      const errorText = await response.text()
      throw new Error(`[${response.status}]: ${errorText || 'Unknown error'}`)
    }

    const data = await response.json()
    // 成功后跳转到结果页
    router.push({ name: 'Result', params: { taskId: data.task_id } })
  } catch (error) {
    submissionError.value = error.message
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="flex flex-col items-center py-6 px-4">
    <div class="w-full max-w-3xl bg-white rounded-lg shadow-xl p-8 dark:bg-gray-900">
      <!-- 标题 -->
      <h1 class="text-4xl font-bold text-center text-gray-800 mb-2 dark:text-gray-400">GRAPE</h1>
      <p class="text-center text-gray-500 mb-8">
        Generator of RNA Aptamers Powered by Activity-guided Evolution
      </p>

      <!-- 表单 -->
      <form @submit.prevent="handleSubmit">
        <!-- 种子序列输入框 -->
        <div class="mb-6">
          <label
            for="seed-sequence"
            class="block text-lg font-medium text-gray-700 dark:text-gray-400 mb-2"
            >Seed Sequences</label
          >
          <textarea
            id="seed-sequence"
            v-model="seedSequence"
            rows="10"
            class="w-full min-h-60 p-3 bg-gray-50 border rounded-lg shadow-sm transition font-mono focus:outline-none focus:ring-2 focus:ring-gray-300 dark:focus:ring-gray-500 dark:border-gray-800 dark:text-gray-400 dark:bg-gray-800"
            :class="validationError ? 'border-red-500 focus:ring-red-500' : 'border-gray-300'"
            placeholder="Enter your seed sequences here, one per line (20 bases long, RNA only)..."
            spellcheck="false"
          ></textarea>
          <!-- 验证错误提示 -->
          <p v-if="validationError" class="mt-2 text-sm text-red-600">{{ validationError }}</p>
        </div>

        <!-- 下拉框和数量输入 -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          <!-- Target 下拉框 -->
          <div>
            <label
              for="target"
              class="block text-lg font-medium text-gray-700 dark:text-gray-400 mb-2"
              >Target</label
            >
            <select
              id="target"
              v-model="target"
              class="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-gray-300 dark:focus:ring-gray-500 dark:border-gray-800 dark:text-gray-400 transition"
            >
              <option v-for="opt in targetOptions" :key="opt" :value="opt">{{ opt }}</option>
            </select>
          </div>
          <!-- Model 下拉框 -->
          <div>
            <label
              for="model"
              class="block text-lg font-medium text-gray-700 dark:text-gray-400 mb-2"
              >Model</label
            >
            <select
              id="model"
              v-model="model"
              class="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-gray-300 dark:focus:ring-gray-500 dark:border-gray-800 dark:text-gray-400 transition"
            >
              <option v-for="opt in modelOptions" :key="opt" :value="opt">w/ {{ opt }}</option>
            </select>
          </div>
          <!-- 数量输入 -->
          <div>
            <label
              for="count"
              class="block text-lg font-medium text-gray-700 dark:text-gray-400 mb-2"
              >Generated Sequences</label
            >
            <input
              type="number"
              id="count"
              v-model="count"
              min="1"
              max="10000"
              class="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-gray-300 dark:focus:ring-gray-500 dark:border-gray-800 dark:text-gray-400 transition"
            />
          </div>
        </div>

        <div class="flex items-center justify-center space-x-4">
          <button
            type="button"
            @click="handleExample"
            class="w-32 px-8 py-3 bg-gray-500 text-white font-semibold rounded-lg shadow-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-opacity-75 transition"
          >
            Example
          </button>
          <button
            type="submit"
            :disabled="isLoading || validationError"
            class="w-32 px-8 py-3 bg-violet-600 text-white font-semibold rounded-lg shadow-md hover:bg-violet-700 focus:outline-none focus:ring-2 focus:ring-violet-500 focus:ring-opacity-75 transition disabled:bg-violet-300 disabled:cursor-not-allowed"
          >
            <span v-if="isLoading">Submitting...</span>
            <span v-else>Submit</span>
          </button>
          <button
            type="button"
            @click="handleReset"
            class="w-32 px-8 py-3 bg-red-500 text-white font-semibold rounded-lg shadow-md hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-400 focus:ring-opacity-75 transition"
          >
            Reset
          </button>
        </div>
      </form>

      <!-- 提交错误显示 -->
      <div
        v-if="submissionError"
        class="mt-8 p-4 rounded-lg bg-red-100 border border-red-400 text-red-800"
      >
        <p>Error {{ submissionError }}</p>
      </div>
      <!-- <div class="mt-8 text-center">
        <router-link to="/" class="text-violet-600 hover:text-violet-800 font-semibold">
          Back to Home
        </router-link>
      </div> -->
    </div>
  </div>
</template>
