<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '@/stores/authStore';
import ProfileAvatar from './ProfileAvatar.vue';

const emit = defineEmits<{
  (e: "openSidebar"): void;
  (e: "search", query: string): void;
}>();

const router    = useRouter();
const authStore = useAuthStore();
const searchValue = ref("");

function handleSearch() {
  emit("search", searchValue.value);
}

function handleLogout() {
  authStore.logout();
  router.push("/login");
}
</script>

<template>
  <div class="flex justify-between items-center px-6 h-14 shadow-md w-full">
    <div class="flex gap-3 items-center">
      <div class="md:hidden">
        <Button icon="pi pi-bars" text @click="emit('openSidebar')"></Button>
      </div>
      <h1 class="text-md font-semibold tracking-tight">NotesInventory</h1>
    </div>

    <div class="bg-gray-100 rounded-full p-2 px-4 text-gray-600 flex items-center">
      <i class="pi pi-search text-gray-400 mr-2 text-sm"></i>
      <InputText
        v-model="searchValue"
        @update:model-value="handleSearch"
        class="border-none bg-transparent text-sm focus:outline-none"
        placeholder="Search notes..."
      />
    </div>

    <!-- No props — ProfileAvatar reads authStore directly -->
    <ProfileAvatar
      @logout="handleLogout"
      @profile="router.push('/profile')"
    />
  </div>
</template>