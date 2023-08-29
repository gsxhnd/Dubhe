import { defineStore } from "pinia";
import { ref } from "vue";

interface sidebar {
  open: boolean;
}
export const useLayoutStore = defineStore("layout", () => {
  const sidebar = ref<sidebar>({ open: true });
  const count = ref(0);
  function increment() {
    count.value++;
  }
  function sidebarToggle() {
    sidebar.value.open = !sidebar.value.open;
  }
  return { sidebar, count, increment, sidebarToggle };
});
