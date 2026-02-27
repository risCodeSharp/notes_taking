import { createApp } from "vue";
import App from "@/App.vue";
import router from "./router";

// import 'primevue/resources/themes/lara-dark-blue/theme.css'
// import 'primevue/resources/primevue.min.css'
// import 'primeicons/primeicons.css'


import PrimeVue from "primevue/config";
import Aura from "@primeuix/themes/aura";
import Tree from 'primevue/tree'
import Sidebar from "primevue/sidebar";
import InputText from "primevue/inputtext";
import Badge from "primevue/badge";
import Drawer from "primevue/drawer";
import Ripple from 'primevue/ripple';
import Editor from 'primevue/editor';
import TabView from "primevue/tabview";
import TabPanel from "primevue/tabpanel";
import PanelMenu from 'primevue/panelmenu';
import Popover from "primevue/popover";
import Menu  from "primevue/menu";
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import Splitter from 'primevue/splitter';
import SplitterPanel from 'primevue/splitterpanel';
import "@/assets/main.css"; // Tailwind base imported
import Button  from "primevue/button";
import Avatar from "primevue/avatar";
const app = createApp(App);

// PrimeVue config with cssLayer
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: ".my-app-dark",
      cssLayer: true,
    },
      ripple: true,
      inputStyle: 'outlined',
      locale: {}
  },
});;


app.component('Tabs', Tabs);
app.component('Popover', Popover);
app.component('TabList', TabList);
app.component('Tab', Tab);
app.component('TabPanels', TabPanels);
app.component('TabPanel', TabPanel);
app.directive('ripple', Ripple);
app.component("Sidebar", Sidebar);
app.component('Tree', Tree);
app.component('Menu', Menu);
app.component("Drawer", Drawer);
app.component("InputText", InputText);
app.component("Avatar", Avatar);
app.component("PanelMenu", PanelMenu);
app.component("Badge", Badge);
app.component("Button", Button);
app.component('Splitter', Splitter);
app.component('SplitterPanel', SplitterPanel);
app.component("Editor", Editor);

app.component("TabView", TabView);
app.component("TabPanel", TabPanel);

app.use(router);
app.mount("#app");