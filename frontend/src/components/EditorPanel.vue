<script lang="ts" setup>
import { MdEditor, MdPreview } from "md-editor-v3";
import type { EditorMode } from "@/types";
import { computed } from "vue";

const props = defineProps<{
  currentMode: EditorMode;
  markdown: string;
  noteName: string;
}>();

const emit = defineEmits<{
  (e: "update:markdown", value: string): void;
}>();

const modelMarkdown = computed({
  get: () => props.markdown,
  set: (val: string) => emit("update:markdown", val),
});
</script>

<template>
  <div class="flex flex-col mr-2 h-full">

    <h2 class="mb-5 px-7 text-3xl pt-3 font-semibold text-gray-600 shrink-0">
      {{ noteName }}
    </h2>

    <!-- Edit Mode -->
    <MdEditor
      v-if="currentMode === 'edit'"
      v-model="modelMarkdown"
      :preview="false"
      language="en-US"
      class="flex-1 min-h-0 mx-3  rounded-2xl mb-6 h-full"
      :style="{ height: '100%' }"
    />

    <!-- Preview Mode -->
    <div
      v-else-if="currentMode === 'preview'"
      class="preview-wrapper flex-1 min-h-0 mb-6 overflow-y-auto rounded-lg py-2 px-4"
    >
      <MdPreview
        :modelValue="modelMarkdown"
        language="en-US"
        class="prose max-w-none px-5 rounded-xl"
      />
    </div>

    <!-- Split Mode -->
    <MdEditor
      v-else
      v-model="modelMarkdown"
      language="en-US"
      class="flex-1 min-h-0 mx-3  rounded-2xl mb-6 h-full"
      :style="{ height: '100%' }"
    />

  </div>
</template>

<style scoped>
.preview-wrapper :deep(.md-editor-preview-wrapper) {
  --md-bk-color: #f3f4f6 !important;
  --md-color: #1f2937;
  --md-border-color: #e5e7eb;
  --md-code-bk-color: #e5e7eb;
  --md-code-color: #374151;
  background-color: #f3f4f6 !important;
  border-radius: 0.5rem;
  padding: 1rem;
}

.preview-wrapper :deep(.md-editor-preview) {
  background-color: #f3f4f6 !important;
}

.preview-wrapper :deep(.md-editor) {
  background-color: #f3f4f6 !important;
}
</style>