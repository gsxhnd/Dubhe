<template>
  <Header class="layout-header" />
  <div class="layout-container">
    <Sidebar
      :is-opened="layoutStore.sidebar.open"
      :menu-items="[{ name: '123', link: '', icon: '', tooltip: '' }]"
      :menu-logo="logo"
    />
  </div>
</template>

<script setup lang="ts">
import { h, defineComponent, ref, Component } from "vue";
import Sidebar from "./Sidebar.vue";
import Header from "./Header.vue";
import { useLayoutStore } from "@/stores/layout";

const layoutStore = useLayoutStore();
const logo = ref("vite.svg");
</script>
