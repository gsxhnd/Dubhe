<template>
  <div class="layout">
    <div class="layout-top">
      <Header class="layout-header" />
    </div>
    <div class="layout-container">
      <Sidebar
        class="layout-sidebar"
        :is-opened="layoutStore.sidebar.open"
        :menu-logo="logo"
      />
      <div class="layout-content">
        <router-view></router-view>
      </div>
    </div>
    <!-- <div class="layout-right"></div> -->
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
  flex-direction: column;
  height: 100vh;
  &-top {
    width: 100%;
  }
  &-container {
    display: flex;
    height: 100%;
  }
}
</style>
