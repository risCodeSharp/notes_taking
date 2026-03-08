<script lang="ts" setup> 
import { ref } from "vue";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import Tab from "primevue/tab";
import TabList from "primevue/tablist";
import TabPanels from "primevue/tabpanels";
import DesktopSidebar from "@/components/DesktopSidebar.vue";
import MobileDrawer from "@/components/MobileDrawer.vue";
import ModeSwitcher from "@/components/ModeSwitcher.vue";
import type { Mode } from "@/types";
import EditorPanel from "@/components/EditorPanel.vue";
import EditorNavbar from "@/components/EditorNavbar.vue";
import TabButton from "@/components/TabButton.vue";
import "md-editor-v3/lib/style.css";
import "md-editor-v3/lib/preview.css";

const tabvalue1 = ref("<div>Hello World!</div><div>PrimeVue <b>Editor</b> Rocks</div>");
const tabvalue2 = ref("Tab 2 content");
const tabvalue3 = ref("Tab 3 content");

const currentMode  = ref<Mode>("Edit");
const activeTab    = ref("0");
const visibleOnMobile = ref(false);

const showSidebar = () => { visibleOnMobile.value = true; };
</script>
<template>
     <!-- Navbar -->
    <EditorNavbar @open-sidebar="showSidebar" class="border-b border-b-gray-200 shrink-0" />

    <!-- Mobile Drawer -->
    <MobileDrawer v-model:visible="visibleOnMobile" />

    <!-- Main Layout -->
    <Splitter
      class="flex-1 min-w-400 overflow-hidden"
      :pt="{
        gutter: 'w-2 bg-gray-200 hover:bg-blue-400 transition-colors duration-150 cursor-col-resize',
      }"
    >
      <!-- Sidebar Panel -->
      <SplitterPanel
        :size="18"
        :minSize="12"
        class="hidden md:flex flex-col overflow-hidden"
      >
        <DesktopSidebar />
      </SplitterPanel>

      <!-- Editor Panel -->
      <SplitterPanel :size="100" :minSize="50" class="overflow-auto bg-gray-100">
        <main class="h-full">
          <div class="ml-2 mx-auto shadow-xl rounded-lg">
            <Tabs v-model="activeTab" v-bind="{} as any" class="pt-2">
              <!-- Tab Headers -->
              <TabList class="flex border-b bg-gray-50 border-gray-300">
                <TabButton @update-tab="(tab: number) => activeTab = String(tab)" :tab-number="0" text="Project Strategy"  :active-tab="Number(activeTab)" />
                <TabButton @update-tab="(tab: number) => activeTab = String(tab)" :tab-number="1" text="Meeting Notes"     :active-tab="Number(activeTab)" />
                <TabButton @update-tab="(tab: number) => activeTab = String(tab)" :tab-number="2" text="go"                :active-tab="Number(activeTab)" />
              </TabList>

              <ModeSwitcher :current-mode="currentMode" @update-mode="(mode: Mode) => currentMode = mode" />

              <!-- Tab Panels -->
              <TabPanels class="p-5 border border-t-0 border-gray-300 rounded-b-md bg-white">
                <EditorPanel :current-mode="currentMode" tab-number="0" v-model:markdown="tabvalue1" noteName="Q4 Project Strategy & Roadmap" />
                <EditorPanel :current-mode="currentMode" tab-number="1" v-model:markdown="tabvalue2" noteName="Test subject" />
                <EditorPanel :current-mode="currentMode" tab-number="2" v-model:markdown="tabvalue3" noteName="Big subject" />
              </TabPanels>
            </Tabs>
          </div>
        </main>
      </SplitterPanel>
    </Splitter>
</template>