<template>
  <div class="border-b border-slate-200">
    <div 
      class="group/header flex items-center px-2 py-3 transition-colors"
      :class="[isEditing ? 'bg-blue-50/30' : 'hover:bg-slate-50/50']"
    >

      <div
        @click="!isEditing && toggle()"
        class="flex-1 flex items-center gap-2.5 text-slate-700 transition-colors duration-150 min-w-0 cursor-pointer"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-slate-400 shrink-0 group-hover/header:text-blue-500 transition-colors" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
          <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" />
        </svg>

        <template v-if="!isEditing">
          <span class="text-[12px] font-bold tracking-wider text-slate-600 group-hover/header:text-slate-900 transition-colors uppercase truncate select-none">
            {{ currentTitle }}
          </span>
        </template>
        
        <template v-else>
          <input
            ref="inputRef"
            v-model="editingTitle"
            @keydown.enter="saveEdit"
            @keydown.esc="cancelEdit"
            @click.stop
            @blur="saveEdit"
            class="text-[12px] font-bold tracking-wider uppercase flex-1 min-w-0 bg-white border border-blue-400 rounded px-2 py-1 outline-none ring-2 ring-blue-100 text-slate-800"
          />
        </template>
      </div>

      <div class="flex items-center gap-1 shrink-0 ml-2">

        <template v-if="isEditing">
          <button @click.stop="saveEdit" class="p-1.5 rounded-md text-emerald-600 hover:bg-emerald-50 transition-all" title="Save">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20 6 9 17 4 12"/>
            </svg>
          </button>
        </template>

        <template v-else-if="showDeleteConfirm">
          <div class="flex items-center gap-1 bg-red-50 px-1.5 py-0.5 rounded-md border border-red-100">
            <span class="text-[10px] text-red-600 font-bold uppercase mr-1">Delete?</span>
            <button @click.stop="deleteNotebook" class="text-[10px] px-2 py-1 rounded bg-red-500 text-white font-bold hover:bg-red-600 transition-all">YES</button>
            <button @click.stop="cancelDelete" class="text-[10px] px-2 py-1 rounded bg-white text-slate-500 border border-slate-200 font-bold hover:bg-slate-50 transition-all">NO</button>
          </div>
        </template>

        <template v-else>
          <div class="flex items-center gap-0.5 opacity-100 md:opacity-0 group-hover/header:opacity-100 transition-opacity">
            <button
              @click.stop="startEditing"
              class="p-2 md:p-1.5 rounded-md text-slate-400 hover:text-blue-600 hover:bg-blue-50 active:bg-blue-100 transition-all"
              title="Rename"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
              </svg>
            </button>

            <button
              @click.stop="confirmDelete"
              class="p-2 md:p-1.5 rounded-md text-slate-400 hover:text-red-600 hover:bg-red-50 active:bg-red-100 transition-all"
              title="Delete"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="3 6 5 6 21 6"/>
                <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/>
                <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2"/>
              </svg>
            </button>
          </div>

          <button @click.stop="toggle" class="p-1.5 text-slate-400 hover:text-slate-600 transition-colors">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="w-4 h-4 transition-transform duration-300" :class="{ 'rotate-180': isOpen }">
              <path fill-rule="evenodd" d="M4.22 6.22a.75.75 0 0 1 1.06 0L8 8.94l2.72-2.72a.75.75 0 1 1 1.06 1.06l-3.25 3.25a.75.75 0 0 1-1.06 0L4.22 7.28a.75.75 0 0 1 0-1.06Z" clip-rule="evenodd"/>
            </svg>
          </button>
        </template>
      </div>
    </div>

    <div
      class="overflow-hidden transition-all duration-300 ease-in-out"
      :class="isOpen ? 'max-h-400 opacity-100' : 'max-h-0 opacity-0'"
    >
      <div class="pb-4 pt-1 flex flex-col gap-0.5">
        <slot />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, nextTick, watch } from 'vue'

const props = defineProps<{ title: string }>()
const emit = defineEmits(['rename', 'delete'])

const isOpen = ref(true)
const isEditing = ref(false)
const editingTitle = ref('')
const currentTitle = ref(props.title)
const showDeleteConfirm = ref(false)
const inputRef = ref<HTMLInputElement | null>(null)

// Sync currentTitle if prop changes from parent
watch(() => props.title, (newVal) => {
  currentTitle.value = newVal
})

const toggle = () => { 
  if (isEditing.value) return
  isOpen.value = !isOpen.value 
}

function startEditing() {
  editingTitle.value = currentTitle.value
  isEditing.value = true
  // We don't want the notebook to collapse/expand when we start editing
  nextTick(() => {
    inputRef.value?.focus()
    inputRef.value?.select()
  })
}

function saveEdit() {
  if (!isEditing.value) return
  
  const trimmed = editingTitle.value.trim()
  if (trimmed && trimmed !== currentTitle.value) {
    currentTitle.value = trimmed
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

function deleteNotebook() {
  showDeleteConfirm.value = false
  emit('delete')
}
</script>