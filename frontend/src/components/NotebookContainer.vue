<template>
  <div class=" border-stone-200/60 mx-1">
    <div class="group/header flex items-center  py-2 transition-colors"
      :class="[isEditing ? 'bg-blue-50/30' : 'hover:bg-slate-50/50']">

      <div @click="!isEditing && toggle()"
        class="flex-1 flex justify-between items-center gap-2.5 text-slate-700 transition-colors duration-150 min-w-0 cursor-pointer">

        <div class="flex justify-between w-full items-center pl-1 gap-1">
          
          <div class="flex gap-2 items-center">
            <button @click.stop="toggle" class="text-stone-400 hover:text-slate-600 transition-colors">
            <ChevronDown class="size-3.5 transition duration-200" :class="{ 'rotate-180': isOpen }" />
          </button>
          <BookType class="size-4.5 text-gray-500/67" />

          <div class="text-[0.84rem]">
          <template v-if="!isEditing">
            <span class=" text-gray-500 uppercase truncate ">
              {{ currentTitle }}
            </span>
          </template>

          <template v-else>
            <input ref="inputRef" v-model="editingTitle" @keydown.enter="saveEdit" @keydown.esc="cancelEdit" @click.stop
              @blur="saveEdit"
              class=" font-bold tracking-wider uppercase flex-1 min-w-0 bg-white border border-blue-400 rounded px-2 py-1 outline-none ring-2 ring-blue-100 text-slate-800" />
          </template>
          </div>
          </div>
          
        </div>
      </div>

      <div class="flex items-center gap-1 shrink-0 ml-2">

        <template v-if="isEditing">
          <button @click.stop="saveEdit" class="p-1.5 rounded-md text-emerald-600 hover:bg-emerald-50 transition-all"
            title="Save">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20 6 9 17 4 12" />
            </svg>
          </button>
        </template>

        <template v-else-if="showDeleteConfirm">
          <div class="flex items-center gap-1 bg-red-50 px-1.5 py-0.5 rounded-md border border-red-100">
            <span class="text-[10px] text-red-600 font-bold uppercase mr-1">Delete?</span>
            <button @click.stop="deleteNotebook"
              class="text-[10px] px-2 py-1 rounded bg-red-500 text-white font-bold hover:bg-red-600 transition-all">YES</button>
            <button @click.stop="cancelDelete"
              class="text-[10px] px-2 py-1 rounded bg-white text-slate-500 border border-slate-200 font-bold hover:bg-slate-50 transition-all">NO</button>
          </div>
        </template>

        <template v-else>
          <div
            class="flex items-center gap-0.5 opacity-100 md:opacity-0 group-hover/header:opacity-100 transition-opacity">
            <button @click.stop="startEditing"
              class="p-2 md:p-1.5 rounded-md text-slate-400 hover:text-blue-600 hover:bg-blue-50 active:bg-blue-100 transition-all"
              title="Rename">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
                <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
              </svg>
            </button>

            <button @click.stop="confirmDelete"
              class="p-2 md:p-1.5 rounded-md text-slate-400 hover:text-red-600 hover:bg-red-50 active:bg-red-100 transition-all"
              title="Delete">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="3 6 5 6 21 6" />
                <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" />
                <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2" />
              </svg>
            </button>
          </div>

        </template>
      </div>
    </div>

    <div class="overflow-hidden transition-all duration-300 ease-in-out"
      :class="isOpen ? 'max-h-400 opacity-100' : 'max-h-0 opacity-0'">
      <div class="pb-4 pt-1 flex flex-col gap-0.5">
        <slot />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { BookType, ChevronDown } from '@lucide/vue';
import { ref, nextTick, watch } from 'vue'

const props = defineProps<{ title: string }>()
const emit = defineEmits(['rename', 'delete', 'toggle'])

const isOpen = ref(false)
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
  emit('toggle')
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