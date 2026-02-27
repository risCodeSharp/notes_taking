<script lang="ts" setup>
import { ref } from 'vue';
import Avatar from 'primevue/avatar';
import Popover from 'primevue/popover';

interface Props {
  name: string;
  role?: string;
  email: string;
}

const props = defineProps<Props>();
const emit = defineEmits(['logout', 'profile']);

const op = ref<InstanceType<typeof Popover> | null>(null);

const toggle = (event: Event) => {
  op.value?.toggle(event);
};

const handleLogout = () => {
  emit('logout');
  op.value?.hide();
};
</script>

<template>
  <button
    type="button"
    class="relative group p-0.5 rounded-full bg-linear-to-tr from-sky-400 to-blue-600 hover:shadow-lg hover:shadow-sky-200/50 transition-all duration-300 active:scale-95"
    @click="toggle"
  >
    <div class="bg-white p-0.5 rounded-full">
        <Avatar 
          :label="props.name.charAt(0)" 
          shape="circle" 
          class="bg-linear-to-br from-sky-500 to-indigo-600 text-white w-9 h-9 font-bold border-2 border-white" 
        />
    </div>
  </button>

  <Popover ref="op" class="overflow-hidden border-none shadow-2xl">
    <div class="w-72 flex flex-col bg-white overflow-hidden rounded-xl">
      
      <div class="relative p-6 bg-linear-to-br from-sky-600 via-blue-600 to-indigo-700 text-center">
        <div class="absolute inset-0 opacity-10 pointer-events-none overflow-hidden">
            <div class="absolute -top-4 -right-4 w-24 h-24 rounded-full bg-white"></div>
            <div class="absolute top-12 -left-8 w-20 h-20 rounded-full bg-white"></div>
        </div>

        <div class="relative z-10 flex flex-col items-center">
            <div class="p-1 rounded-full bg-white/20 backdrop-blur-md mb-3">
                <Avatar 
                :label="props.name.charAt(0)" 
                shape="circle" 
                class="bg-white text-sky-700 w-14 h-14 text-2xl font-black shadow-lg" 
                />
            </div>
            <h3 class="text-white font-bold text-lg tracking-tight">{{ props.name }}</h3>
            <div class="mt-1 px-3 py-0.5 bg-sky-400/30 backdrop-blur-sm border border-sky-300/30 rounded-full">
                <span class="text-[10px] font-bold text-sky-50 uppercase tracking-widest">
                {{ props.role ?? 'Editor' }}
                </span>
            </div>
        </div>
      </div>

      <div class="p-3 bg-white">
        <div class="px-3 py-2 mb-2">
            <p class="text-[11px] font-bold text-gray-400 uppercase tracking-widest mb-1">EMAIL</p>
            <p class="text-sm text-gray-600 truncate font-medium">{{ props.email }}</p>
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