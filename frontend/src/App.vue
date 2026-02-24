<script lang="ts" setup>
import { ref } from "vue";
import Drawer from "primevue/drawer";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import { MdEditor } from "md-editor-v3";
import "md-editor-v3/lib/style.css";
import "md-editor-v3/lib/preview.css";

import Tab from "primevue/tab";
import TabList from "primevue/tablist";
import TabPanel from "primevue/tabpanel";
import TabPanels from "primevue/tabpanels";

const searchValue = ref("");
const tabvalue1 = ref("<div>Hello World!</div><div>PrimeVue <b>Editor</b> Rocks</div>");
const tabvalue2 = ref("Tab 2 content");
const tabvalue3 = ref("Tab 3 content");

const activeTab = ref("0");
const visible = ref(false);

const showSidebar = () => {
  visible.value = true;
};

const addNewTab = () => {
  // Implement logic to add new tab (optional)
  alert("Add new tab clicked!");
};
</script>

<template>
  <div class="flex flex-col h-screen">
    <!-- Navbar -->
    <div class="flex justify-between items-center px-5 h-16 shadow-md w-full bg-white">
      <div class="flex gap-2 items-center">
        <div class="md:hidden">
          <Button icon="pi pi-bars" text @click="showSidebar" />
        </div>
        <h1 class="font-bold text-lg">Notes Inventory</h1>
      </div>

      <!-- Search -->
      <div class="bg-gray-100 rounded-full p-2 px-4 text-gray-600 flex items-center">
        <i class="pi pi-search mr-3"></i>
        <InputText
          v-model="searchValue"
          class="border-none bg-transparent focus:outline-none"
          placeholder="Search"
        />
      </div>

      <div>accounts</div>
    </div>

    <!-- Mobile Drawer -->
    <Drawer v-model:visible="visible" class="md:hidden" position="left">
      <template #header>
        <h2 class="font-bold text-lg">Menu</h2>
      </template>
      <ul class="mt-4 space-y-2">
        <li class="cursor-pointer hover:text-primary">Dashboard</li>
        <li class="cursor-pointer hover:text-primary">Notes</li>
        <li class="cursor-pointer hover:text-primary">Settings</li>
      </ul>
    </Drawer>

    <!-- Main Layout -->
    <div class="flex flex-1 overflow-hidden">
      <!-- Desktop Sidebar -->
      <aside class="hidden md:block bg-red-500 w-45 overflow-y-auto">
        <h2 class="font-bold text-lg p-4 text-white">Menu</h2>
        <ul class="mt-4 space-y-2 p-4 text-white">
          <li class="cursor-pointer hover:text-gray-200">Dashboard</li>
          <li class="cursor-pointer hover:text-gray-200">Notes</li>
          <li class="cursor-pointer hover:text-gray-200">Settings</li>
        </ul>
      </aside>

      <!-- Main Content -->
      <main class="flex-1 bg-gray-50 overflow-auto p-5">
        <div class="bg-white rounded-t-md shadow-sm">
          <Tabs v-model="activeTab" v-bind="{} as any">
            <!-- Tab Headers -->
            <TabList class="flex border-b border-gray-300 border ">
              <Tab
                value="0"
                class="border px-4 py-2 cursor-pointer text-sm font-medium -mb-px border-b-2 transition-colors"
                :class="activeTab === '0' ? 'border-blue-600 text-blue-600 font-semibold' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'"
              >
                Q4 Project Strategy
              </Tab>
              <Tab
                value="1"
                class="border px-4 py-2 cursor-pointer text-sm font-medium -mb-px border-b-2 transition-colors"
                :class="activeTab === '1' ? 'border-blue-600 text-blue-600 font-semibold' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'"
              >
                Meeting Notes
              </Tab>
              <Tab
                value="2"
                class="border px-4 py-2 cursor-pointer text-sm font-medium -mb-px border-b-2 transition-colors"
                :class="activeTab === '2' ? 'border-blue-600 text-blue-600 font-semibold' : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'"
              >
                Personal Ideas
              </Tab>
              <!-- Plus tab for new tab -->
              <div
                @click="addNewTab"
                class="ml-4 px-3 py-2 cursor-pointer text-gray-400 hover:text-gray-700 select-none"
                title="Add new tab"
              >
                +
              </div>
            </TabList>

            <!-- Tab Panels -->
            <TabPanels class="p-5 border border-t-0 border-gray-300 rounded-b-md bg-white">
              <TabPanel value="0">
                <p class="mb-4 text-gray-700 font-semibold text-lg">
                  Q4 Project Strategy & Roadmap
                </p>
                <MdEditor v-model="tabvalue1" language="en-US" />
              </TabPanel>

              <TabPanel value="1">
                <p class="mb-4 text-gray-700 font-semibold text-lg">Meeting Notes</p>
                <MdEditor v-model="tabvalue2" language="en-US" />
              </TabPanel>

              <TabPanel value="2">
                <p class="mb-4 text-gray-700 font-semibold text-lg">Personal Ideas</p>
                <MdEditor v-model="tabvalue3" language="en-US" />
              </TabPanel>
            </TabPanels>
          </Tabs>
        </div>
      </main>
    </div>
  </div>
</template>