<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  taskId: {
    type: String,
    required: true,
  },
})

const taskResponse = ref(null)
const isLoading = ref(true)
const fetchError = ref(null)
const resultSequences = ref('')
let pollingInterval = null

// 格式化日期
const formatDate = (dateString) => {
  if (!dateString) return 'N/A'
  return new Date(dateString).toLocaleString()
}

// 下载结果文件
const downloadResult = async () => {
  try {
    const response = await fetch(`/api/tasks/${props.taskId}/download`)
    if (!response.ok) {
      const errorText = await response.text()
      throw new Error(`[${response.status}]: ${errorText || 'Unknown error'}`)
    }
    const blob = await response.blob()
    const url = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.style.display = 'none'
    a.href = url
    a.download = `generation_${props.taskId}.txt`
    document.body.appendChild(a)
    a.click()
    window.URL.revokeObjectURL(url)
    document.body.removeChild(a)
  } catch (e) {
    fetchError.value = e.message
  }
}

// 获取并显示结果内容
const fetchAndShowResultContent = async () => {
  try {
    const response = await fetch(`/api/tasks/${props.taskId}/download`)
    if (!response.ok) {
      const errorText = await response.text()
      throw new Error(`[${response.status}]: ${errorText || 'Unknown error'}`)
    }
    const blob = await response.blob()
    resultSequences.value = await blob.text()
  } catch (e) {
    fetchError.value = e.message
  }
}

// 轮询任务状态
const fetchTaskStatus = async () => {
  try {
    const response = await fetch(`/api/tasks/${props.taskId}`)
    if (!response.ok) {
      const errorText = await response.text()
      throw new Error(`[${response.status}]: ${errorText || 'Unknown error'}`)
    }
    const data = await response.json()
    taskResponse.value = data

    // 检查任务是否完成或失败
    if (data.type === 'Completed' || data.type === 'Failed') {
      clearInterval(pollingInterval)
      if (data.type === 'Completed') {
        await fetchAndShowResultContent()
      }
    }
  } catch (e) {
    fetchError.value = e.message
    clearInterval(pollingInterval)
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  fetchTaskStatus()
  pollingInterval = setInterval(fetchTaskStatus, 3000)
})

onUnmounted(() => {
  clearInterval(pollingInterval)
})
</script>

<template>
  <div class="flex flex-col items-center py-10 px-4">
    <div class="w-full max-w-3xl bg-white rounded-lg shadow-xl p-8">
      <h1 class="text-3xl font-bold text-center text-gray-800 mb-6">Generation Results</h1>

      <div v-if="isLoading" class="text-center text-gray-500">
        <p>Loading task details...</p>
      </div>
      <!-- 查询结果失败 -->
      <div
        v-else-if="fetchError"
        class="p-4 rounded-md bg-red-100 border border-red-400 text-red-800"
      >
        <p>Error {{ fetchError }}</p>
      </div>
      <!-- 成功和服务器通信，获得任务状态反馈 -->
      <div v-else-if="taskResponse">
        <div class="grid grid-cols-2 gap-4 mb-6 border-b border-gray-300 pb-4">
          <div>
            <strong class="text-gray-600">Status: </strong>
            <span
              class="font-semibold"
              :class="{
                'text-violet-600': taskResponse.type === 'Pending',
                'text-blue-600': taskResponse.type === 'Processing',
                'text-green-600': taskResponse.type === 'Completed',
                'text-red-600': taskResponse.type === 'Failed',
              }"
              >{{ taskResponse.type }}</span
            >
          </div>
          <div>
            <strong class="text-gray-600">Submitted: </strong>
            {{ formatDate(taskResponse.data.upload_time) }}
          </div>
          <div>
            <strong class="text-gray-600">Started: </strong>
            {{ formatDate(taskResponse.data.start_time) }}
          </div>
          <div>
            <strong class="text-gray-600">Finished: </strong>
            {{ formatDate(taskResponse.data.end_time) }}
          </div>
        </div>

        <div
          v-if="taskResponse.type === 'Pending' || taskResponse.type === 'Processing'"
          class="text-center py-8"
        >
          <!-- 等待动画 -->
          <svg
            class="animate-spin h-8 w-8 text-violet-600 mx-auto mb-4"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
          </svg>
          <p v-if="taskResponse.type === 'Pending'" class="text-lg text-gray-600">
            Your task is queued at position
            <span class="font-bold">{{ taskResponse.data.position || 0 }}</span
            >. Please wait...
          </p>
          <p v-else class="text-lg text-gray-600">Your task is being processed. Please wait...</p>
          <p class="text-sm text-gray-400">The page will update automatically.</p>
          <p class="text-sm text-gray-400 mb-4">
            You can bookmark this page to view your results later.
          </p>
        </div>
        <!-- 任务失败 -->
        <div
          v-else-if="taskResponse.type === 'Failed'"
          class="p-4 rounded-md bg-red-100 border border-red-400 text-red-800"
        >
          <p>
            <strong>Task Failed:</strong>
            {{ taskResponse.data.error || 'An unknown error occurred.' }}
          </p>
        </div>
        <!-- 任务成功 -->
        <div v-else-if="taskResponse.type === 'Completed'">
          <h2 class="text-xl font-semibold text-gray-700 mb-4">Generated Sequences</h2>
          <!-- 展示生成的序列 -->
          <textarea
            :value="resultSequences"
            readonly
            rows="10"
            class="w-full min-h-60 p-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-gray-300 transition font-mono"
          ></textarea>
          <div class="mt-4 text-center">
            <button
              @click="downloadResult"
              class="px-6 py-2 bg-green-600 text-white font-semibold rounded-lg shadow-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-opacity-75 transition"
            >
              Download
            </button>
          </div>
        </div>
      </div>
      <!-- 返回链接 -->
      <div class="mt-8 text-center space-x-4">
        <router-link to="/submit" class="text-violet-600 hover:text-violet-800 font-semibold">
          Submit New Task
        </router-link>
        <span class="text-gray-400">|</span>
        <router-link to="/" class="text-violet-600 hover:text-violet-800 font-semibold">
          Back to Home
        </router-link>
      </div>
    </div>
  </div>
</template>
