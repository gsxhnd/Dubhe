<template>
  <div class="layout-header">
    <Button
      class="collapse"
      icon="pi pi-align-justify"
      @click="layoutStore.sidebarToggle"
      text
      outlined
    />
  </div>
</template>

<script setup lang="ts">
import Button from "primevue/button";
import { useLayoutStore } from "@/stores/layout";
const layoutStore = useLayoutStore();
</script>

<style lang="less" scoped>
.layout-header {
  height: 50px;
  background: @layout-header-bg-color;
  display: flex;
  justify-content: left;
  .collapse {
    width: 75px;
    :deep span {
      font-size: 20px;
    }
  }
}
</style>
