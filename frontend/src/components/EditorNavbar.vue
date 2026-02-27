<script lang="ts" setup>
import { ref } from 'vue';
import ProfileAvatar from './ProfileAvatar.vue';


const emit = defineEmits<{
  (e: "openSidebar"): void;
}>();
const searchValue = ref("");

// emit when submitting on search 
</script>
<template>
  <div class="flex justify-between items-center px-6 h-14 shadow-md w-full">
    <div class="flex gap-3 items-center">
      <div class="md:hidden">
        <Button icon="pi pi-bars" text @click="emit('openSidebar')"></Button>
      </div>
      <h1 class="text-md font-semibold tracking-tight">NotesInventory</h1>
    </div>

    <!-- Search -->
    <div
      class="bg-gray-100 rounded-full p-2 px-4 text-gray-600 flex items-center"
    >
      <i class="pi pi-search text-gray-400 mr-2 text-sm"></i>
      <InputText
        v-model="searchValue"
        class="border-none bg-transparent text-sm focus:outline-none"
        placeholder="Search notes..."
      />
    </div>

    <ProfileAvatar name="xyz" email="abc@gmail.com"/>
  </div>
</template>
