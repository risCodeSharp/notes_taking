<script lang="ts" setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '@/stores/authStore';
import ProfileAvatar from './ProfileAvatar.vue';
import logo from '@/assets/logo.png'
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
  <div class="flex justify-between items-center px-4 h-16 shadow-md w-full">
    <div class="flex gap-3 items-center">
      <div class="md:hidden">
        <Button icon="pi pi-bars" text @click="emit('openSidebar')"></Button>
      </div>
      <RouterLink to="/">
    <img :src="logo" class="w-40 object-cover" alt="NotesInvertory" />
</RouterLink>
    </div>

    <div class="flex gap-6 ">
    <div class="bg-gray-100 border border-stone-300 rounded-full p-2 px-4 text-gray-600 flex items-center">
      <i class="pi pi-search text-stone-600 mr-2 text-[0.68rem] "></i>
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
  </div>
</template>