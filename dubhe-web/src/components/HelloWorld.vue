<script setup lang="ts">
import { ref } from "vue";

defineProps<{ msg: string }>();

const count = ref(0);
</script>

<template>
  <div class="Hello">hello world</div>
</template>

<style scoped lang="less"></style>
