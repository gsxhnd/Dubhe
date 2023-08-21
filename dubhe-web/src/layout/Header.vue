<template>
  <div class="layout-header">
    <Button
      icon="pi pi-arrow-right"
      @click="layoutStore.sidebarToggle"
      label="Submit"
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
}
</style>
