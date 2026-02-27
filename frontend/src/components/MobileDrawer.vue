<script lang="ts" setup>
import { computed } from "vue";
import SidebarNav from "./SidebarNav.vue";

const props = defineProps({ visible: Boolean });
const emit = defineEmits(["update:visible"]);

const modelVisible = computed({
  get: () => props.visible,
  set: (val) => emit("update:visible", val),
});
</script>

<template>
  <Drawer v-model:visible="modelVisible" class="md:hidden w-68 p-3" position="left">
    <template #header>
      <!-- intentionally empty — header is inside SidebarNav -->
    </template>
    <SidebarNav />
  </Drawer>
</template>