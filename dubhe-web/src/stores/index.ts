import { createPinia, defineStore } from "pinia";

export const testStore = defineStore("test", {
  state: () => {
    return {
      cars: {},
      price: 0,
    };
  },
});
