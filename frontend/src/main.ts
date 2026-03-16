import { createApp } from "vue";
import App from "@/App.vue";
import router from "./router";

import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";

import Tree from "primevue/tree";
import Sidebar from "primevue/sidebar";
import InputText from "primevue/inputtext";
import Badge from "primevue/badge";
import Drawer from "primevue/drawer";
import Ripple from "primevue/ripple";
import Editor from "primevue/editor";
import TabView from "primevue/tabview";
import TabPanel from "primevue/tabpanel";
import PanelMenu from "primevue/panelmenu";
import Popover from "primevue/popover";
import Menu from "primevue/menu";
import Tabs from "primevue/tabs";
import TabList from "primevue/tablist";
import Tab from "primevue/tab";
import TabPanels from "primevue/tabpanels";
import Splitter from "primevue/splitter";
import SplitterPanel from "primevue/splitterpanel";
import Button from "primevue/button";
import Avatar from "primevue/avatar";

import { createPinia } from "pinia";

import "@/assets/main.css";

const app = createApp(App);

app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: ".my-app-dark",
      cssLayer: true
    }
  },
  ripple: true,
  inputStyle: "outlined"
});

// Components
app.component("Tabs", Tabs);
app.component("Popover", Popover);
app.component("TabList", TabList);
app.component("Tab", Tab);
app.component("TabPanels", TabPanels);
app.component("TabPanel", TabPanel);
app.component("Sidebar", Sidebar);
app.component("Tree", Tree);
app.component("Menu", Menu);
app.component("Drawer", Drawer);
app.component("InputText", InputText);
app.component("Avatar", Avatar);
app.component("PanelMenu", PanelMenu);
app.component("Badge", Badge);
app.component("Button", Button);
app.component("Splitter", Splitter);
app.component("SplitterPanel", SplitterPanel);
app.component("Editor", Editor);
app.component("TabView", TabView);

// Directives
app.directive("ripple", Ripple);

app.use(createPinia());
app.use(router);

app.mount("#app");