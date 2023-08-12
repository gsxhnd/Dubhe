<script setup lang="ts">
import { RouterView } from "vue-router";
import { ref, onMounted, onUnmounted } from "vue";
import { testStore } from "@/stores/index";

const test = testStore();
</script>

<template>
  <router-view class="router-view" />
</template>

<style scoped lang="less"></style>
