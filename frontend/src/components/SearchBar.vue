<script lang="ts" setup>
import { MagnifyingGlassIcon } from '@heroicons/vue/24/outline';
</script>

<template>
  <div class="mx-10 max-w-md w-3/4">
    <label for="search" class="sr-only">Search notes</label>
    
    <div class="group flex items-center h-[46px] gap-3 px-4 rounded-full border border-gray-200 bg-white transition-all duration-200 focus-within:border-blue-500 focus-within:ring-2 focus-within:ring-blue-100 shadow-sm">
      
      <MagnifyingGlassIcon class="h-5 w-5 text-gray-400 group-focus-within:text-blue-500 transition-colors" />
      
      <input 
        id="search"
        type="text" 
        placeholder="Search notes, tags or content..." 
        class="w-full h-full bg-transparent text-sm text-gray-700 placeholder-gray-400 outline-none" 
      />
      
      <kbd class="hidden md:inline-flex items-center px-1.5 font-sans text-[10px] font-medium text-gray-400 border border-gray-200 rounded">
        ⌘K
      </kbd>
    </div>
  </div>
</template>