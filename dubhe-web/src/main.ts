import { createApp } from "vue";
import { router } from "@/router";
import { createPinia } from "pinia";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import Tooltip from "primevue/tooltip";
import "primevue/resources/themes/lara-light-indigo/theme.css";
import "./style.less";

const pinia = createPinia();

const app = createApp(App);
app.use(PrimeVue);
app.use(router);
app.use(pinia);
app.directive("tooltip", Tooltip);

app.mount("#app");
