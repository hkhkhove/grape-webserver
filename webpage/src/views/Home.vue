<script setup>
import frameworkImage from '@/assets/images/grape-lm-framework.png'
</script>
<template>
  <div class="flex flex-col items-center py-6 px-4">
    <div class="w-full max-w-3xl bg-white rounded-lg shadow-xl p-8 dark:bg-gray-900">
      <!-- 标题 -->
      <h1 class="text-4xl font-bold text-center text-gray-800 mb-2 dark:text-gray-400">GRAPE-LM</h1>
      <p class="text-center text-gray-500 mb-8">
        Generator of RNA Aptamers Powered by activity-guided Evolution and Language Model
      </p>

      <!-- 方法介绍 -->
      <div class="mb-4">
        <h2 class="text-2xl font-semibold text-gray-800 mb-4 dark:text-gray-400">Overview</h2>
        <div class="prose max-w-none text-gray-700 dark:text-gray-500">
          <p class="mb-4 text-justify">
            GRAPE-LM<sup class="font-medium">1</sup> is a novel AI-driven framework designed for the
            rapid evolution of RNA aptamers. Unlike traditional approaches, which require
            labor-intensive, multi-round screening, GRAPE-LM enables one-round evolution by
            integrating a Transformer-based conditional autoencoder with nucleic acid language
            models<sup class="font-medium">2,3</sup>. The framework is guided by CRISPR/Cas-based
            aptamer screening data obtained from biologically relevant intracellular environments.
          </p>
          <div class="mb-4">
            <div class="bg-gray-50 p-4 rounded-lg dark:bg-gray-800">
              <img
                :src="frameworkImage"
                alt="GRAPE-LM Framework Diagram"
                class="w-full h-auto rounded-lg shadow-sm"
              />
              <p class="text-sm text-gray-500 mt-2 text-center italic">
                Overview of the GRAPE-LM framework for RNA aptamer evolution.
              </p>
            </div>
          </div>
        </div>
        <!-- <p class="">
          The method starts with user-provided seed sequences and iteratively generates new RNA
          aptamer candidates.
        </p> -->
      </div>

      <!-- 跳转按钮 -->
      <!-- <div class="text-center">
        <router-link
          to="/submit"
          class="inline-block px-8 py-3 bg-violet-600 text-white font-semibold rounded-lg shadow-md hover:bg-violet-700 focus:outline-none focus:ring-2 focus:ring-violet-500 focus:ring-opacity-75 transition transform hover:scale-105"
        >
          Start Generating Aptamers
        </router-link>
      </div> -->

      <!-- 参考文献 -->
      <div class="mt-10">
        <h2 class="text-2xl font-semibold text-gray-800 mb-4 dark:text-gray-400">References</h2>
        <div class="bg-gray-50 p-4 rounded-lg dark:bg-gray-800">
          <div class="space-y-3 text-sm text-gray-700 dark:text-gray-500">
            <div class="border-l-4 border-violet-500 pl-4">
              <p class="font-medium">
                <span class="font-bold">1.</span> Jun Zhang, Ju Zhang, Shaoxuan Tang, Chuancheng
                Liu, Yushan Cai, Hao Zeng, Xiangjie Meng, Yang Zhang, Yu Wang. One Round Evolution
                of RNA Aptamers with GRAPE-LM.
              </p>
              <p class="text-gray-600 italic dark:text-gray-600">(Submitted).</p>
            </div>

            <div class="border-l-4 border-violet-500 pl-4">
              <p class="font-medium">
                <span class="font-bold">2.</span> Shen, T. et al. Accurate RNA 3D structure
                prediction using a language model-based deep learning approach.
              </p>
              <p class="text-gray-600 italic dark:text-gray-600">
                Nature Methods 21, 2287-2298 (2024).
              </p>
            </div>

            <div class="border-l-4 border-violet-500 pl-4">
              <p class="font-medium">
                <span class="font-bold">3.</span> Nguyen, E. et al. Sequence modeling and design
                from molecular to genome scale with Evo.
              </p>
              <p class="text-gray-600 italic dark:text-gray-600">Science 386, eado9336 (2024).</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
