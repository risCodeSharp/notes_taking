<script lang="ts" setup>
import { File, FilePen, SquarePen, Trash } from '@lucide/vue';
import { ref, nextTick } from 'vue'

const props = defineProps<{ name: string }>()
const emit = defineEmits(['rename', 'delete'])

const showDeleteConfirm = ref(false)
const inputRef = ref<HTMLInputElement | null>(null)
const isEditing = ref(false)
const editingName = ref('')
const currentName = ref(props.name)

function startEditing() {
  editingName.value = currentName.value
  isEditing.value = true
  nextTick(() => {
    inputRef.value?.focus()
    inputRef.value?.select()
  })
}

function saveEdit() {
  const trimmed = editingName.value.trim()
  if (trimmed && trimmed !== currentName.value) {
    currentName.value = trimmed
    emit('rename', trimmed)
  }
  isEditing.value = false
}

function cancelEdit() {
  isEditing.value = false
}

function confirmDelete() {
  showDeleteConfirm.value = true
}

function cancelDelete() {
  showDeleteConfirm.value = false
}

function deleteItem() {
  showDeleteConfirm.value = false
  emit('delete')
}
</script>

<template>
  <div
    class="group flex items-center gap-3 px-3 mr-1 py-2  border mx-1 cursor-pointer border-stone-200 rounded-md transition-all duration-150 select-none"
    :class="[
      isEditing ? 'bg-blue-50/60' : 'hover:bg-slate-100/80 active:bg-slate-100/50',
      showDeleteConfirm ? 'bg-red-50' : ''
    ]"
  >
    <template v-if="!isEditing && !showDeleteConfirm">
      <File class="size-4.5 text-gray-400"/>

      <span class="flex-1 text-[0.78rem] text-stone-600 font-normal truncate group-hover:text-slate-900 transition-colors duration-150 mr-5">
        {{ currentName }}
      </span>
      <div class="relative">
      <div class="absolute flex items-center gap-1 opacity-100  group-hover:opacity-100 transition-all duration-150 bottom-0 right-0 translate-2.5">
        <button
          @click.stop="startEditing"
          class=" rounded-full text-stone-400 p-1.5 hover:text-blue-400 hover:bg-blue-100 active:scale-95 transition-all"
          title="Rename"
        >
          <SquarePen class="size-3"/>
        </button>

        <button
          @click.stop="confirmDelete"
          class=" rounded-full p-1.5 text-stone-400 hover:text-red-400 hover:bg-red-100 active:scale-95 transition-all"
          title="Delete"
        >
          <Trash class="size-3"/>
        </button>
      </div>
      </div>
    </template>

    <template v-else-if="isEditing">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-blue-500 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/>
        <polyline points="14 2 14 8 20 8"/>
      </svg>
      <input
        ref="inputRef"
        v-model="editingName"
        @keydown.enter="saveEdit"
        @keydown.esc="cancelEdit"
        @blur="saveEdit"
        class="flex-1 min-w-0 text-[13px] text-slate-800 bg-white border border-blue-400 rounded px-2 py-1 outline-none ring-2 ring-blue-100"
      />
      <div class="flex items-center gap-1 shrink-0">
        <button @click="saveEdit" class="p-1.5 text-emerald-600 hover:bg-emerald-50 rounded">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        </button>
      </div>
    </template>

    <template v-else-if="showDeleteConfirm">
      <div class="flex flex-1 items-center gap-2">
        <span class="text-[13px] text-red-600 font-sm">Delete {{currentName}}?</span>
      </div>
      <div class="flex items-center gap-1 shrink-0">
        <button 
          @click="cancelDelete" 
          class="px-2.5 py-1 text-[11px] font-semibold text-slate-500 hover:bg-slate-200 rounded-md transition-colors"
        >
          Cancel
        </button>
        <button 
          @click="deleteItem" 
          class="px-2.5 py-1 text-[11px] font-semibold text-white bg-red-500 hover:bg-red-600 rounded-md shadow-sm transition-colors"
        >
          Confirm
        </button>
      </div>
    </template>
  </div>
</template>