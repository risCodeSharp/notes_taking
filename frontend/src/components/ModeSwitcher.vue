<script lang="ts" setup>
import type { Mode } from "@/types";
import {
  EyeIcon,
  PencilIcon,
  BookmarkSquareIcon,
} from "@heroicons/vue/24/outline";

defineProps<{ currentMode: Mode }>();
const emit = defineEmits<{
  (e: "updateMode", mode: Mode): void;
}>();

const buttons: { label: Mode; icon: any }[] = [
  { label: "Edit", icon: PencilIcon },
  { label: "Preview", icon: EyeIcon },
  { label: "Split", icon: null }, // Uses the pi-icon 
];
</script>
<!-- TODO: REMOVE THIS WITH THE PRIMEVUE: SelectButton for better ui/ui experierce-->
<template>
  <div class="flex items-center justify-between b bg-white px-4 py-2">
    
    <div class="flex items-center bg-gray-100 p-1 px-1.5 rounded-xl ">
      <button
        v-for="btn in buttons"
        :key="btn.label"
        type="button"
        @click="emit('updateMode', btn.label)"
        :class="[
          'flex items-center gap-2 px-4 py-1.5 text-sm font-medium rounded-lg transition-all duration-200',
          currentMode === btn.label
            ? 'bg-white text-blue-600 shadow-sm ring-1 ring-black/5'
            : 'text-gray-500 hover:text-gray-700 hover:bg-gray-200/50'
        ]"
      >
        <component v-if="btn.icon" :is="btn.icon" class="w-4 h-4" />
        <i v-else class="pi pi-objects-column text-sm"></i>
        
        {{ btn.label }}
      </button>
    </div>

    <button
      type="button"
      class="group flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-blue-700 active:scale-95 transition-all focus:outline-none focus:ring-1 focus:ring-blue-500 focus:ring-offset-2"
    >
      Save
      <BookmarkSquareIcon class="w-4 h-4 transition-transform duration-200 group-hover:scale-110" />
    </button>
  </div>
</template>