<template>
  <div class="layout">
    <Sidebar
      class="layout-sidebar"
      :is-opened="layoutStore.sidebar.open"
      :menu-items="[{ name: '123', link: '', icon: '', tooltip: '' }]"
      :menu-logo="logo"
    />
    <div class="layout-right">
      <Header class="layout-header" />
      <div class="layout-context">
        <router-view></router-view>
      </div>
    </div>
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

<style lang="less" scoped>
.layout {
  display: flex;
  height: 100vh;
  // &-sidebar {
  // }
  &-right {
    width: 100%;
  }
  &-context {
    height: 100%;
  }
}
</style>
