<script lang="ts" setup>
import type { EditorMode} from "@/types";
import { EyeIcon, PencilIcon, BookmarkSquareIcon } from "@heroicons/vue/24/outline";

defineProps<{
  currentMode: EditorMode
  isSaving?:   boolean   // shows spinner on Save button while API call is in flight
}>();

const emit = defineEmits<{
  (e: "update-mode", mode: EditorMode): void
  (e: "save"):                   void
}>();

const buttons: { label: EditorMode; icon: any }[] = [
  { label: "edit",    icon: PencilIcon },
  { label: "preview", icon: EyeIcon    },
  { label: "split",   icon: null       },
];
</script>

<template>
  <div class="flex items-center justify-between bg-white px-4 py-2">

    <div class="flex items-center bg-gray-100 p-1 px-1.5 rounded-xl">
      <button
        v-for="btn in buttons"
        :key="btn.label"
        type="button"
        @click="emit('update-mode', btn.label)"
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

    <!-- Save button — spinner while saving -->
    <button
      type="button"
      :disabled="isSaving"
      @click="emit('save')"
      class="group flex items-center gap-2 rounded-lg bg-blue-600 px-4 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-blue-700 active:scale-95 transition-all focus:outline-none focus:ring-1 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-60 disabled:cursor-not-allowed"
    >
      <span>{{ isSaving ? 'Saving…' : 'Save' }}</span>
      <i v-if="isSaving" class="pi pi-spin pi-spinner w-4 h-4"></i>
      <BookmarkSquareIcon v-else class="w-4 h-4 transition-transform duration-200 group-hover:scale-110" />
    </button>

  </div>
</template>