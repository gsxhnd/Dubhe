<template>
  <div class="header">
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

<style lang="less" scoped></style>
