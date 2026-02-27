<script lang="ts" setup>
import { MdEditor, MdPreview } from "md-editor-v3";
import type { Mode } from "@/types";
import { computed } from "vue";

const props = defineProps<{
  currentMode: Mode;
  tabNumber: string;
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
  <TabPanel :value="tabNumber">
    <div class="flex flex-col">

      <h2 class="mb-6 text-xl font-semibold text-gray-800">
        {{ noteName }}
      </h2>

      <!-- Editor Mode -->
      <MdEditor
        v-if="currentMode === 'Edit'"
        v-model="modelMarkdown"
        :preview="false"
        language="en-US"
        class="min-h-100"
      />

      <!-- Preview Mode -->
      <div
        v-else-if="currentMode === 'Preview'"
        class="rounded-lg bg-white max-h-100 overflow-y-auto py-2"
      >
        <MdPreview
          :modelValue="modelMarkdown"
          language="en-US"
          class="prose max-w-none"
        />
      </div>

      <!-- Mixed / other Mode -->
      <MdEditor
        v-else
        v-model="modelMarkdown"
        language="en-US"
        class="min-h-100"
      />

    </div>
  </TabPanel>
</template>