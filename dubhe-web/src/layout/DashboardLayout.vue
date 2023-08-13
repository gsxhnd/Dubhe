<template>
  <div class="layout-header">
    <Button icon="pi pi-arrow-right" @click="visible = true" label="Submit" />
  </div>
  <div class="layout-container">
    <Sidebar
      :is-search="true"
      :menu-items="[{ name: '123', link: '', icon: '', tooltip: '' }]"
    />
    <div class="layout-header"></div>
  </div>
</template>

<script setup lang="ts">
import { h, defineComponent, ref, Component } from "vue";
import Button from "primevue/button";
import Sidebar from "./Sidebar.vue";

const visible = ref(false);
</script>
