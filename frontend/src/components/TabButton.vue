<script setup lang="ts">
import { XMarkIcon } from "@heroicons/vue/24/outline";

defineProps<{
  activeTab:  number
  text:       string
  tabNumber:  number
  isDirty?:   boolean   // shows unsaved dot when true
}>();

const emit = defineEmits<{
  (e: 'update-tab', tab: number): void
  (e: 'close-tab'):               void
}>();
</script>

<template>
  <button
    type="button"
    @click="emit('update-tab', tabNumber)"
    class="relative px-5 py-3 cursor-pointer text-sm transition-all duration-300 flex items-center gap-3 border-r border-gray-200 group overflow-hidden rounded-t-xl"
    :class="activeTab === tabNumber
      ? 'text-blue-700 bg-white shadow-sm'
      : 'text-gray-500 hover:text-blue-500 bg-gray-50 hover:bg-white'
    "
  >
    <!-- Active underline -->
    <div v-if="activeTab === tabNumber" class="absolute bottom-0 left-0 right-0 h-0.5 bg-blue-400"></div>

    <!-- Icon -->
    <i
      class="pi pi-file transition-all duration-300"
      :class="activeTab === tabNumber ? 'text-blue-600 scale-110' : 'text-gray-400 group-hover:text-blue-400'"
    ></i>

    <!-- Text -->
    <span class="font-medium tracking-tight whitespace-nowrap">{{ text }}</span>

    <!-- Unsaved dot -->
    <span v-if="isDirty" class="w-1.5 h-1.5 rounded-full bg-orange-400 shrink-0" title="Unsaved changes"></span>

    <!-- Close button -->
    <span
      @click.stop="emit('close-tab')"
      class="ml-auto p-1 rounded-md text-gray-400 hover:text-red-500 hover:bg-red-50 transition-colors duration-200"
    >
      <XMarkIcon class="w-4 h-4" />
    </span>
  </button>
</template>