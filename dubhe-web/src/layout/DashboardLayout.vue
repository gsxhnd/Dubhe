<template>
  <div class="layout-header">
    <Button icon="pi pi-arrow-right" @click="open = !open" label="Submit" />
  </div>
  <div class="layout-container">
    <Sidebar
      :is-opened="open"
      :menu-items="[{ name: '123', link: '', icon: '', tooltip: '' }]"
      :menu-logo="logo"
    />
    <div class="layout-header"></div>
  </div>
</template>

<script setup lang="ts">
import { h, defineComponent, ref, Component } from "vue";
import Button from "primevue/button";
import Sidebar from "./Sidebar.vue";

const logo = ref("vite.svg");
const open = ref(true);
</script>
