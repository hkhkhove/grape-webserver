<template>
  <div class="flex flex-col items-center py-10 px-4">
    <div class="w-full max-w-3xl bg-white rounded-lg shadow-xl p-8 text-center">
      <h1 class="text-6xl font-bold text-violet-600 mb-4">404</h1>
      <h2 class="text-3xl font-bold text-gray-800 mb-2">Not Found</h2>
      <p class="text-gray-500 mb-8">Sorry, the page you are looking for does not exist.</p>
      <router-link to="/" class="text-violet-600 hover:text-violet-800 font-semibold">
        &larr; Back to Home
      </router-link>
    </div>
  </div>
</template>
