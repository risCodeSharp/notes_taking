import { createApp } from "vue";
import App from "@/App.vue";
import router from "./router";

import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";

import Sidebar from "primevue/sidebar";
import InputText from "primevue/inputtext";
import Avatar from "primevue/avatar";
import Badge from "primevue/badge";
import Drawer from "primevue/drawer";
import Ripple from 'primevue/ripple';
import Editor from 'primevue/editor';
import TabView from "primevue/tabview";
import TabPanel from "primevue/tabpanel";
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import "@/assets/main.css"; // Tailwind base imported

const app = createApp(App);

// PrimeVue config with cssLayer
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: ".my-app-dark",
      cssLayer: true,      // IMPORTANT for Tailwind + PrimeVue
    },
      ripple: true, // enable ripple globally if you want
  inputStyle: 'outlined', // optional
  locale: {} // optional
  },
});


app.component('Tabs', Tabs);
app.component('TabList', TabList);
app.component('Tab', Tab);
app.component('TabPanels', TabPanels);
app.component('TabPanel', TabPanel);
app.directive('ripple', Ripple);
app.component("Sidebar", Sidebar);
app.component("Drawer", Drawer);
app.component("InputText", InputText);
app.component("Avatar", Avatar);
app.component("Badge", Badge);
app.component("Editor", Editor);

app.component("TabView", TabView);
app.component("TabPanel", TabPanel);

app.use(router);
app.mount("#app");