<script setup>
import { ref } from 'vue'

const openContact = ref(false)
</script>

<template>
  <div class="flex flex-col min-h-screen bg-gray-100 dark:bg-gray-800">
    <header>
      <nav class="min-w-sm bg-white shadow-sm dark:bg-gray-900 dark:text-gray-400">
        <div class="max-w-7xl min-w-sm flex flex-wrap items-center justify-between mx-auto p-4">
          <!-- logo -->
          <router-link to="/" class="flex items-center space-x-3">
            <svg
              class="h-8 w-8"
              viewBox="0 0 47.5 47.5"
              id="svg2"
              version="1.1"
              xml:space="preserve"
              xmlns="http://www.w3.org/2000/svg"
              xmlns:cc="http://creativecommons.org/ns#"
              xmlns:dc="http://purl.org/dc/elements/1.1/"
              xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
              xmlns:svg="http://www.w3.org/2000/svg"
            >
              <defs id="defs6">
                <clipPath clipPathUnits="userSpaceOnUse" id="clipPath16">
                  <path d="M 0,38 38,38 38,0 0,0 0,38 Z" id="path18" />
                </clipPath>
              </defs>

              <g id="g10" transform="matrix(1.25,0,0,-1.25,0,47.5)">
                <g id="g12">
                  <g clip-path="url(#clipPath16)" id="g14">
                    <g id="g20" transform="translate(10.999,25)">
                      <path
                        d="m 0,0 c -0.15,0 -0.303,0.034 -0.446,0.105 -4.381,2.191 -7.484,7.527 -8.502,10.579 -0.175,0.524 0.109,1.09 0.633,1.265 0.525,0.176 1.09,-0.11 1.265,-0.633 C -6.173,8.685 -3.362,3.799 0.448,1.895 0.942,1.647 1.143,1.047 0.896,0.553 0.72,0.202 0.367,0 0,0"
                        id="path22"
                        style="fill: #77b255; fill-opacity: 1; fill-rule: nonzero; stroke: none"
                      />
                    </g>

                    <g id="g24" transform="translate(27,8)">
                      <path
                        d="m 0,0 c 0,-3.865 -3.135,-7 -7,-7 -3.865,0 -7,3.135 -7,7 0,3.865 3.135,7 7,7 3.865,0 7,-3.135 7,-7"
                        id="path26"
                        style="fill: #553788; fill-opacity: 1; fill-rule: nonzero; stroke: none"
                      />
                    </g>

                    <g id="g28" transform="translate(18,22)">
                      <path
                        d="m 0,0 c 0,-3.865 -3.135,-7 -7,-7 -3.865,0 -7,3.135 -7,7 0,3.865 3.135,7 7,7 3.865,0 7,-3.135 7,-7"
                        id="path30"
                        style="fill: #9266cc; fill-opacity: 1; fill-rule: nonzero; stroke: none"
                      />
                    </g>

                    <g id="g32" transform="translate(27,25)">
                      <path
                        d="m 0,0 c 0,-3.865 -3.135,-7 -7,-7 -3.865,0 -7,3.135 -7,7 0,3.865 3.135,7 7,7 3.865,0 7,-3.135 7,-7"
                        id="path34"
                        style="fill: #aa8dd8; fill-opacity: 1; fill-rule: nonzero; stroke: none"
                      />
                    </g>

                    <g id="g36" transform="translate(35,19)">
                      <path
                        d="m 0,0 c 0,-3.865 -3.135,-7 -7,-7 -3.865,0 -7,3.135 -7,7 0,3.865 3.135,7 7,7 3.865,0 7,-3.135 7,-7"
                        id="path38"
                        style="fill: #744eaa; fill-opacity: 1; fill-rule: nonzero; stroke: none"
                      />
                    </g>

                    <g id="g40" transform="translate(17,11)">
                      <path
                        d="m 0,0 c 0,-3.865 -3.135,-7 -7,-7 -3.865,0 -7,3.135 -7,7 0,3.865 3.135,7 7,7 3.865,0 7,-3.135 7,-7"
                        id="path42"
                        style="fill: #744eaa; fill-opacity: 1; fill-rule: nonzero; stroke: none"
                      />
                    </g>

                    <g id="g44" transform="translate(26,16)">
                      <path
                        d="m 0,0 c 0,-3.865 -3.135,-7 -7,-7 -3.865,0 -7,3.135 -7,7 0,3.865 3.135,7 7,7 3.865,0 7,-3.135 7,-7"
                        id="path46"
                        style="fill: #9266cc; fill-opacity: 1; fill-rule: nonzero; stroke: none"
                      />
                    </g>

                    <g id="g48" transform="translate(37,8)">
                      <path
                        d="m 0,0 c 0,-3.865 -3.135,-7 -7,-7 -3.865,0 -7,3.135 -7,7 0,3.865 3.135,7 7,7 3.865,0 7,-3.135 7,-7"
                        id="path50"
                        style="fill: #9266cc; fill-opacity: 1; fill-rule: nonzero; stroke: none"
                      />
                    </g>
                  </g>
                </g>
              </g>
            </svg>
            <span class="text-2xl font-semibold">GRAPE-LM</span>
          </router-link>

          <!-- menu -->
          <div class="w-auto">
            <ul class="flex flex-row font-medium p-0 mt-0 space-x-4">
              <li>
                <router-link
                  to="/"
                  class="block py-0 px-3 text-gray-700 dark:text-gray-400 hover:text-blue-700 dark:hover:text-blue-400"
                  >Home</router-link
                >
              </li>
              <li>
                <router-link
                  to="/submit"
                  class="block py-0 px-3 text-gray-700 dark:text-gray-400 hover:text-blue-700 dark:hover:text-blue-400"
                  >Web Server</router-link
                >
              </li>
              <li
                class="relative"
                @mouseenter="openContact = true"
                @mouseleave="openContact = false"
              >
                <button
                  class="flex items-center justify-between w-full py-0 px-3 text-gray-700 dark:text-gray-400 hover:text-blue-700 dark:hover:text-blue-400"
                >
                  Contact Us
                  <svg
                    class="w-2.5 h-2.5 ms-2.5"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 10 6"
                  >
                    <path
                      stroke="currentColor"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="m1 1 4 4 4-4"
                    />
                  </svg>
                </button>
                <!-- Outside div keeps seamless to avoid triggering the mouseleave event -->
                <div
                  v-show="openContact"
                  class="absolute w-40 left-1/2 transform -translate-x-1/2 font-normal"
                >
                  <div
                    class="mt-2 bg-white divide-y divide-gray-100 rounded-lg shadow-sm dark:bg-gray-700 dark:divide-gray-700"
                  >
                    <ul class="block py-2 text-sm text-center text-gray-700 dark:text-gray-400">
                      <li>
                        <a
                          href="mailto:junzhang@szu.edu.cn"
                          class="flex items-center justify-center space-x-3 px-4 py-2 hover:text-blue-700 dark:hover:text-blue-400"
                          ><svg
                            class="w-6 h-6 transition-colors"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            xmlns="http://www.w3.org/2000/svg"
                          >
                            <path
                              d="M13.025 17H3.707l5.963-5.963L12 12.83l2.33-1.794 1.603 1.603a5.463 5.463 0 0 1 1.004-.41l-1.808-1.808L21 5.9v6.72a5.514 5.514 0 0 1 1 .64V5.5A1.504 1.504 0 0 0 20.5 4h-17A1.504 1.504 0 0 0 2 5.5v11A1.5 1.5 0 0 0 3.5 18h9.525c-.015-.165-.025-.331-.025-.5s.01-.335.025-.5zM3 16.293V5.901l5.871 4.52zM20.5 5c.009 0 .016.005.025.005L12 11.57 3.475 5.005c.009 0 .016-.005.025-.005zm-2 8a4.505 4.505 0 0 0-4.5 4.5 4.403 4.403 0 0 0 .05.5 4.49 4.49 0 0 0 4.45 4h.5v-1h-.5a3.495 3.495 0 0 1-3.45-3 3.455 3.455 0 0 1-.05-.5 3.498 3.498 0 0 1 5.947-2.5H20v.513A2.476 2.476 0 0 0 18.5 15a2.5 2.5 0 1 0 1.733 4.295A1.497 1.497 0 0 0 23 18.5v-1a4.555 4.555 0 0 0-4.5-4.5zm0 6a1.498 1.498 0 0 1-1.408-1 1.483 1.483 0 0 1-.092-.5 1.5 1.5 0 0 1 3 0 1.483 1.483 0 0 1-.092.5 1.498 1.498 0 0 1-1.408 1zm3.5-.5a.5.5 0 0 1-1 0v-3.447a3.639 3.639 0 0 1 1 2.447z"
                            />
                            <path fill="none" d="M0 0h24v24H0z" />
                          </svg>
                          <span>Email</span></a
                        >
                      </li>
                    </ul>
                  </div>
                </div>
              </li>
            </ul>
          </div>
        </div>
      </nav>
    </header>
    <main>
      <router-view />
    </main>
    <footer class="mt-auto text-center">
      <aside class="text-sm text-gray-700 dark:text-gray-500">
        <p>
          Copyright © 2025 BioAILab |
          <a href="https://beian.miit.gov.cn/" target="_blank">粤ICP备2025419210号</a>
        </p>
      </aside>
    </footer>
  </div>
</template>
