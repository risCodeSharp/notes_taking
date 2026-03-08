<template>
  <div class="relative inline-block dropdown-user-wrapper">
    <button 
      @click="isDropdownOpen = !isDropdownOpen"
      class="flex text-sm bg-gray-800 rounded-full md:me-0 focus:ring-4 focus:ring-gray-200 transition-all" 
      type="button"
    >
      <span class="sr-only">Open user menu</span>
      <img class="w-8 h-8 rounded-full" src="https://ui-avatars.com/api/?name=Joseph+McFall&background=0D8ABC&color=fff" alt="user photo">
    </button>

    <transition
      enter-active-class="transition duration-100 ease-out"
      enter-from-class="transform scale-95 opacity-0"
      enter-to-class="transform scale-100 opacity-100"
      leave-active-class="transition duration-75 ease-in"
      leave-from-class="transform scale-100 opacity-100"
      leave-to-class="transform scale-95 opacity-0"
    >
      <div 
        v-show="isDropdownOpen"
        class="absolute right-0 mt-2 z-50 bg-white border border-gray-200 rounded-xl shadow-xl w-72 overflow-hidden"
      >
        <div class="p-3">
          <div class="flex items-center px-3 py-2 space-x-3 text-sm bg-gray-50 rounded-lg">
            <img class="w-8 h-8 rounded-full" src="https://ui-avatars.com/api/?name=Joseph+McFall&background=0D8ABC&color=fff" alt="Avatar">
            <div class="flex-1 min-w-0">
              <div class="font-bold text-gray-900 truncate">Joseph McFall</div>
              <div class="truncate text-gray-500 text-xs">name@flowbite.com</div>
            </div>
            <span class="bg-blue-100 text-blue-700 text-[10px] font-bold px-1.5 py-0.5 rounded">PRO</span>
          </div>
        </div>

        <ul class="px-2 pb-2 text-sm font-medium text-gray-700">
          <li>
            <a href="#" class="inline-flex items-center w-full p-2 hover:bg-gray-100 rounded-lg transition-colors">
              <svg class="w-4 h-4 me-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-width="2" d="M7 17v1a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1v-1a3 3 0 0 0-3-3h-4a3 3 0 0 0-3 3Zm8-9a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"/>
              </svg>
              Account
            </a>
          </li>

          <li class="my-1 border-t border-gray-100"></li>

          <li>
            <a href="#" class="inline-flex items-center w-full p-2 text-red-600 hover:bg-red-50 rounded-lg transition-colors">
              <svg class="w-4 h-4 me-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H8m12 0-4 4m4-4-4-4M9 4H7a3 3 0 0 0-3 3v10a3 3 0 0 0 3 3h2"/>
              </svg>
              Sign out
            </a>
          </li>
        </ul>
      </div>
    </transition>
  </div>
</template>

<script lang="ts" setup>
import { type Ref,ref, onMounted, onUnmounted } from 'vue'

const isDropdownOpen:Ref<boolean> = ref(false)

const closeOnOutsideClick = (e: any) => {
  if (!e.target.closest('.dropdown-user-wrapper')) {
    isDropdownOpen.value = false
  }
}

onMounted(() => window.addEventListener('click', closeOnOutsideClick))
onUnmounted(() => window.removeEventListener('click', closeOnOutsideClick))
</script>