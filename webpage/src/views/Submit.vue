<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { nanoid } from 'nanoid'

const router = useRouter()

const seedSequence = ref('')
const target = ref('CD3ε')
const model = ref('RNA-FM')
const genNum = ref('')
const isLoading = ref(false)
const submissionError = ref(null)

const maxGenNum = computed(() => {
  const seedNum = seedSequence.value.split('\n').filter((line) => line.trim() !== '').length
  return Math.min((seedNum * (seedNum - 1)) / 2, 10000) // 生成数量上限为10000
})

const defaultState = {
  seedSequence: '',
  target: 'CD3ε',
  model: 'RNA-FM',
  genNum: '',
}

const targetOptions = ['CD3ε', 'RBD', 'c-Myc']
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
      return `Error on line ${i + 1}: Sequence must be 20 bases long.`
    }
  }
  if (lines.length < 2) {
    return 'At least 2 seed sequences are required.'
  }
  return null // 验证通过
})

// 填充示例数据
function handleExample() {
  seedSequence.value = `UGCGACAAGCUUCGCUAUGG
CGUCGACUGAAAUCCUCUUC
GUACGACGUUCAUGUUUUUU
CGGUGAGUGUGACUCAUGCG
GGACUUAGUGCUGCCCGCCG
AGAGCGGUAUAGCUAGGUGU
CGGUGAGUGUGGCUCAUGUG
UGCGACUAGCUUCGCUAUGG
CGGUGAGUGUGGCUCAUGCA
UGCGACAAGCUUCUCUAUGG
AGACCGACGGCGACACAAGC
UGCAACAAGCUUCGCUAUGG
GGUCGUCGGGCGACCGGCCG
CGGGCGCCACCGGCGCGCAG
UGCGACAAGCUUCGCUAUGA
AGCGGCGGGACGGACGCGGA
CCCAGACCGGCGGCGCCGAG
UGGACAACGACUGCGCUAUG
UGGUCCCGGGCAGCCCGGCC
ACGUCAAGUUUCCGCUAUGG
CUGUGGGAUGUGACAUGCGG
CCGGAUCCCUGCCCGAGACC
UGCGUCGCGGCGCGACGGCG
GUCGUCGGCGGCCGCGCAGG
GUCGCCAGGGGACCGGCCGG
AGGGUCGCCGGGGACGGCCG
UCCGCUCCGAUACUCGCCUC
GUGGCACGGGACGGCGCGGC
GCGGCUGACCGGGACGCGGA
CGCGUACAGUUCCCGCCGCU
GUCGGCGGCGCUCCUCGGAC
GUACGGUCCGCUCUCCUCGG
CCAGCCGUCGGCGCGACCGG
UCGUCGUGGCGCGGGCACGG
CGGCAGGCCGCCUCGACGCC
CCAUGAGUCACAUCUCUCAG
CGGUCGACCGACGGGGCGCA
UAAUGCGUGGCGCGAGGCGU
CGCUAGUCUGCGCGCGAGGC
CGCCGCCGGCCGCCGACAUA
AUGCGUCGGCCCGACUUGGC
GCCGGCCGCGCGACGCGCUC
UUCUGGCGCGUGGCGCUCGC
CCCGGCCACCGAGCGCGCUG
CGGCGACUUCGACGGGCCAC
UCGGCGGCGCCAGGUCACGG
GGCAACCGGCGUGCGCCAGC
CUCUGCGCCGCAUCGGUCCG
CCCUCGGGCCCCCGGUGGGA
GCGCGGUCCGGUGGGCGCCU
GCCGCCGUCGUGGCGUCCCG
CGGGCUUCCCGGCAGCCCAG
CGGAGGCCGCCGGCGCGAGU
CCUGCGCGCGGUAGGUCCGA
UCCGCCCUUCGGUCGUACCU
AUACGCCCUGUCGCUCACCG
UCGGCUGACUGCCGGUGGGU
CUCGCGUGCUGCCGGUUGAG
CCCGGCGCCGCCGGUCUGCC
UCAGCGGCGCCUCCUCGACC
CCGCGUCGGAGGAGGCUGGC
UCCGCCCGUCGGCUACUUCG
UCCGCCGCAGGGCGUACGGC
UCGCGACGCCACCCACCGGA
GCCCGCCUGUGCGUGCGACG
UCGGUCCCGGCGGCGGCGAC
GUUUCGGGGGUGGUGUCGCG
UCAGCCUCGGCGCUGUCCUG
CGGUCCGCCCAUCGGCAGGC
CCGGGAGUCGCGCCUAGGCU
CCAGGUCGUCGGCCGCCGGA
GGCCGGACCGAGCGGGCCGC
GCGGGCCCCCUCUGUGGCCA
CGGCGCUUGGCAUCCGGCCG
CGGUUCCGGCGGCCGUCGAU
CGCGGUCACGCGGCGCGACC
GGUGGGUGGCUUCUUGCCGA
ACGGUGGUGGCGUUCGUCGU
GCGCGCGGCGGGUACCCGGU
CGCCGCGCGCGACGUUCUGC
GAGCCCGGUGCCUGUCGUCG
CGCCGGAGAGCGGGCCUCGA
UCGUGCCCCGGCGCUGGGAC
CGGUCCGCGGACUGCGGGCC
UCCUAUGGGUGGGUGGUCCG
ACGCGGCCACCCACUACGGA
UGCGCGCGGCGUCGGUCCGG
GUACGCGCGACAGCGGGCGU
GGGCCCGUCGGUCGCGGCGA
GCGAACCGCGUCGGUGGGCC
ACGUCAACUGGAAUCUCUUC
GGCCUGUACACCCCCGGCGG
AAGUACCGCCCUGCGCGGAA
GCUAGUUGGUCGUCGGAUCU
CCCGCGCUCGUCGAGCGCGG
GGACCCGACGGCCCACAAGA
AUGUGAUCGCCUCGAGUCUA
AGACCCGACGUCGGACAAAG
GGUUACCCGCCGCGCGCAAG
GCGGGGCAAGGGUACCGGUG
`
  target.value = 'CD3ε'
  genNum.value = '100'
}

// 重置表单
function handleReset() {
  seedSequence.value = defaultState.seedSequence
  target.value = defaultState.target
  model.value = defaultState.model
  genNum.value = defaultState.genNum
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
  formData.append('task_name', `${target.value}_${model.value}`)
  formData.append('seed_seqs', seedSequence.value)
  formData.append('target', target.value)
  formData.append('model', model.value)
  formData.append('gen_num', genNum.value)

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
    router.push({
      name: 'Result',
      params: { taskId: data.task_id },
    })
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
      <h1 class="text-4xl font-bold text-center text-gray-800 mb-2 dark:text-gray-400">GRAPE-LM</h1>
      <p class="text-center text-gray-500 mb-8">
        Generator of RNA Aptamers Powered by activity-guided Evolution and Language Model
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
            required
          ></textarea>
          <!-- 验证错误提示 -->
          <p v-if="validationError" class="mt-2 text-sm text-red-600">{{ validationError }}</p>
        </div>

        <!-- 下拉框和数量输入 -->
        <div class="flex justify-between space-x-4 mb-6">
          <!-- Target 下拉框 -->
          <div class="w-full">
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
          <div class="w-full">
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
          <div class="w-full">
            <label
              for="count"
              class="block text-lg font-medium text-gray-700 dark:text-gray-400 mb-2"
              >Generated Sequences</label
            >
            <input
              type="number"
              id="count"
              v-model="genNum"
              min="1"
              :max="maxGenNum"
              required
              class="w-full p-3 border border-gray-300 rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-gray-300 dark:focus:ring-gray-500 dark:border-gray-800 dark:text-gray-400 transition"
            />
          </div>
        </div>

        <!-- 按钮 -->
        <div class="flex items-center justify-center space-x-4">
          <button
            type="button"
            @click="handleExample"
            class="w-30 py-3 bg-gray-500 text-white font-semibold rounded-lg shadow-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-opacity-75 transition"
          >
            Example
          </button>
          <button
            type="submit"
            :disabled="isLoading || validationError"
            class="px-8 py-3 bg-violet-600 text-white font-semibold rounded-lg shadow-md hover:bg-violet-700 focus:outline-none focus:ring-2 focus:ring-violet-500 focus:ring-opacity-75 transition disabled:bg-violet-300 disabled:cursor-not-allowed"
          >
            <span v-if="isLoading">Submitting...</span>
            <span v-else>Submit</span>
          </button>
          <button
            type="button"
            @click="handleReset"
            class="w-30 py-3 bg-red-500 text-white font-semibold rounded-lg shadow-md hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-red-400 focus:ring-opacity-75 transition"
          >
            Reset
          </button>
        </div>
      </form>

      <!-- 提交错误显示 -->
      <div
        v-if="submissionError"
        class="mt-6 p-4 rounded-lg bg-red-100 border border-red-400 text-red-800"
      >
        <p>Error {{ submissionError }}</p>
      </div>

      <!-- 说明 -->
      <div class="mt-6 p-4 text-gray-700">
        <p class="text-sm mb-2 dark:text-gray-400">
          <strong>Note:</strong>
        </p>
        <ul class="text-sm dark:text-gray-500 space-y-1.5 list-disc list-inside">
          <li>
            Online generation may take several minutes due to CPU-only computation. For faster
            processing or to explore other language models (which require GPU acceleration), please
            run our
            <a
              href="https://github.com/tansaox2008123/GRAPE-LM"
              target="_blank"
              rel="noopener noreferrer"
              class="dark:text-gray-500 underline hover:text-blue-700 dark:hover:text-blue-400"
              >open-source code</a
            >
            locally.
          </li>
          <li>Gaussian noise is applied during generation to introduce stochastic variation.</li>
          <li>
            Currently, the method is optimized for 20-base seed sequences and generates 20-base
            aptamers candidates. Future updates will extend support to longer sequences and broader
            applications.
          </li>
        </ul>
      </div>
      <!-- <div class="mt-8 text-center">
        <router-link to="/" class="text-violet-600 hover:text-violet-800 font-semibold">
          Back to Home
        </router-link>
      </div> -->
    </div>
  </div>
</template>
