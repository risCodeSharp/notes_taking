<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue';
import Avatar from 'primevue/avatar';
import Popover from 'primevue/popover';
import { useAuthStore } from '@/stores/authStore';
import { useRouter } from 'vue-router';

const emit = defineEmits(['logout', 'profile']);

const authStore = useAuthStore();
const router    = useRouter();

// Fetch on mount so the avatar is always current,
// even if the parent hasn't called fetchMe yet.
onMounted(() => authStore.fetchMe());

const op = ref<InstanceType<typeof Popover> | null>(null);

const toggle = (event: Event) => op.value?.toggle(event);

const handleLogout = () => {
  emit('logout');
  op.value?.hide();
};

// Derive initials from name first, then email local-part, fallback to '?'
const initial = computed(() => {
  const src = authStore.user?.email ?? '';
  return src.trim().charAt(0).toUpperCase() || '?';
});

const displayName = computed(() => authStore.user?.email ?? '');

const displayEmail = computed(() => authStore.user?.email ?? '');
</script>

<template>
  <button
    type="button"
    class="relative group p-0.5 rounded-full bg-linear-to-tr from-sky-400 to-blue-600 hover:shadow-lg hover:shadow-sky-200/50 transition-all duration-300 active:scale-95"
    @click="toggle"
  >
    <div class="bg-white p-0.5 rounded-full">
      <Avatar
        :label="initial"
        shape="circle"
        class="bg-linear-to-br from-sky-500 to-indigo-600 text-white font-bold border-2 border-white"
        style="width: 36px; height: 36px;"
      />
    </div>
  </button>

  <Popover ref="op" class="overflow-hidden border-none shadow-2xl">
    <div class="w-72 flex flex-col bg-white overflow-hidden rounded-xl">

      <!-- Header -->
      <div class="relative p-6 bg-linear-to-br from-sky-600 via-blue-600 to-indigo-700 text-center">
        <div class="absolute inset-0 opacity-10 pointer-events-none overflow-hidden">
          <div class="absolute -top-4 -right-4 w-24 h-24 rounded-full bg-white"></div>
          <div class="absolute top-12 -left-8 w-20 h-20 rounded-full bg-white"></div>
        </div>

        <div class="relative z-10 flex flex-col items-center">
          <div class="p-1 rounded-full bg-white/20 backdrop-blur-md mb-3">
            <Avatar
              :label="initial"
              shape="circle"
              class="bg-white text-sky-700 font-black shadow-lg"
              style="width: 56px; height: 56px; font-size: 1.5rem;"
            />
          </div>

          <!-- Show name if available, otherwise email, otherwise loading skeleton -->
          <template v-if="authStore.meLoading && !displayName">
            <div class="h-4 w-28 bg-white/30 rounded-full animate-pulse mb-2"></div>
            <div class="h-3 w-16 bg-white/20 rounded-full animate-pulse"></div>
          </template>
          <template v-else>
            <h3 class="text-white font-bold text-lg tracking-tight truncate max-w-50">
              {{ displayName || displayEmail }}
            </h3>
            <div class="mt-1 px-3 py-0.5 bg-sky-400/30 backdrop-blur-sm border border-sky-300/30 rounded-full">
              <span class="text-[10px] font-bold text-sky-50 uppercase tracking-widest">Editor</span>
            </div>
          </template>
        </div>
      </div>

      <!-- Body -->
      <div class="p-3 bg-white">
        <div class="px-3 py-2 mb-2">
          <p class="text-[11px] font-bold text-gray-400 uppercase tracking-widest mb-1">EMAIL</p>
          <template v-if="authStore.meLoading && !displayEmail">
            <div class="h-3.5 w-40 bg-gray-200 rounded-full animate-pulse"></div>
          </template>
          <p v-else class="text-sm text-gray-600 truncate font-medium">
            {{ displayEmail || '—' }}
          </p>
        </div>

        <div class="space-y-1">
          <button
            @click="emit('profile')"
            class="w-full flex items-center gap-3 px-3 py-2.5 text-sm text-gray-700 hover:bg-sky-50 rounded-lg transition-colors group"
          >
            <i class="pi pi-user text-sky-500 group-hover:scale-110 transition-transform"></i>
            <span class="font-medium">Account Settings</span>
          </button>

          <div class="my-2 border-t border-gray-100"></div>

          <button
            @click="handleLogout"
            class="w-full flex items-center gap-3 px-3 py-2.5 text-sm text-red-500 hover:bg-red-50 rounded-lg transition-colors group"
          >
            <i class="pi pi-power-off group-hover:rotate-12 transition-transform"></i>
            <span class="font-bold">Logout</span>
          </button>
        </div>
      </div>

    </div>
  </Popover>
</template>

<style scoped>
:deep(.p-popover-content) {
  padding: 0;
  border-radius: 12px;
}
</style>