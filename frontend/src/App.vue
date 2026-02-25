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
import { EyeIcon, PencilIcon } from "@heroicons/vue/24/outline";
const searchValue = ref("");
const tabvalue1 = ref(
  "<div>Hello World!</div><div>PrimeVue <b>Editor</b> Rocks</div>",
);
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
    <div
      class="flex justify-between items-center px-5 h-16 shadow-md w-full bg-white"
    >
      <div class="flex gap-2 items-center">
        <div class="md:hidden">
          <Button icon="pi pi-bars" text @click="showSidebar" />
        </div>
        <h1 class="font-bold text-lg">Notes Inventory</h1>
      </div>

      <!-- Search -->
      <div
        class="bg-gray-100 rounded-full p-2 px-4 text-gray-600 flex items-center"
      >
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
        <div class="bg-white rounded-md shadow-sm">
          <Tabs v-model="activeTab" v-bind="{} as any">
            <!-- Tab Headers -->
            <TabList class="flex border-b border-gray-300">
              <Tab
                value="0"
                class="px-4 py-2 cursor-pointer text-sm font-medium -mb-px border-r border-gray-200 border-b-2 transition-colors rounded-t-xl"
                :class="
                  activeTab === '0'
                    ? 'border-b-blue-800 text-gray-800 font-semibold bg-white'
                    : 'border-b-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 bg-gray-50'
                "
                @click="activeTab = '0'"
              >
                <i
                  class="pi pi-file mr-2"
                  :class="activeTab === '0' ? 'text-blue-700' : 'text-sky-300'"
                ></i>
                Q4 Project Strategy
              </Tab>

              <Tab
                value="1"
                class="px-4 py-2 cursor-pointer text-sm font-medium -mb-px border-r border-gray-200 border-b-2 transition-colors rounded-t-xl"
                :class="
                  activeTab === '1'
                    ? 'border-b-blue-800 text-gray-800 font-semibold bg-white'
                    : 'border-b-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 bg-gray-50'
                "
                @click="activeTab = '1'"
              >
                <i
                  class="pi pi-file mr-2"
                  :class="activeTab === '1' ? 'text-blue-700' : 'text-sky-300'"
                ></i>
                Meeting Notes
              </Tab>

              <Tab
                value="2"
                class="px-4 py-2 cursor-pointer text-sm font-medium -mb-px border-b-2 transition-colors rounded-t-xl"
                :class="
                  activeTab === '2'
                    ? 'border-b-blue-800 text-gray-800 font-semibold bg-white'
                    : 'border-b-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 bg-gray-50'
                "
                @click="activeTab = '2'"
              >
                <i
                  class="pi pi-file mr-2"
                  :class="activeTab === '2' ? 'text-blue-700' : 'text-sky-300'"
                ></i>
                Personal Ideas
              </Tab>
            </TabList>

            <div class="flex border-gray-300 border-b justify-between px-2 text-gray-700">
              <div class="flex justify-normal gap-2 py-2">
                <button class="text-sm py-2 px-4 hover:bg-gray-100 rounded-xl">
                  <PencilIcon class="w-4 h-4 inline mr-2" />Edit
                </button>
                <button class="py-2 px-4 hover:bg-gray-100 rounded-xl">
                  <EyeIcon class="w-4 h-4 inline mr-2" />Preview
                </button>
                <button class="py-2 px-4 hover:bg-gray-100 rounded-xl">
                  <i class="pi pi-objects-column mr-2"></i>Split
                </button>
              </div>
              <button>2</button>
            </div>

            <!-- Tab Panels -->
            <TabPanels
              class="p-5 border border-t-0 border-gray-300 rounded-b-md bg-white"
            >
              <TabPanel value="0">
                <h4
                  class="mb-4 text-gray-700 font-extrabold text-4xl my-10 ml-10"
                >
                  Q4 Project Strategy & Roadmap
                </h4>
                <MdEditor v-model="tabvalue1" language="en-US" />
              </TabPanel>

              <TabPanel value="1">
                <p class="mb-4 text-gray-700 font-semibold text-lg">
                  Meeting Notes
                </p>
                <MdEditor v-model="tabvalue2" language="en-US" />
              </TabPanel>

              <TabPanel value="2">
                <p class="mb-4 text-gray-700 font-semibold text-lg">
                  Personal Ideas
                </p>
                <MdEditor v-model="tabvalue3" language="en-US" />
              </TabPanel>
            </TabPanels>
          </Tabs>
        </div>
        <Button> HEllo</Button>
      </main>
    </div>
  </div>
</template>
