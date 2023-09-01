<template>
  <div class="sidebar" :class="isOpened ? 'open' : 'close'">
    <div class="logo">
      <template v-if="menuLogo">
        <img src="@/assets/vue.svg" class="menu-logo icon" />
      </template>
      <template v-else>
        <i class="bx icon"></i>
      </template>
      <div class="logo-name">{{ menuTitle }}</div>
    </div>
    <div class="item">
      <SidebarNavMenu />
    </div>
  </div>
</template>

<script setup lang="ts">
import SidebarNavMenu from "./SidebarNavMenu.vue";

defineProps({
  menuLogo: { type: String, default: "" },
  menuTitle: { type: String, default: "Dubhe" },
  isOpened: { type: Boolean, default: true },
  isUsedVueRouter: { type: Boolean, default: false },
  bgColor: {
    type: String,
    default: "#11101d",
  },
});
</script>

<style lang="less" scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  // position: fixed;
  left: 0;
  top: 0;
  min-height: min-content;
  width: 78px;
  background: @layout-sidebar-bg-color;
  z-index: 99;
  transition: all 0.5s ease;

  .logo {
    display: flex;
    margin: 6px 14px 0 14px;
    justify-content: center;
  }

  .item {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    flex-grow: 1;
    max-height: calc(100% - 60px);
  }

  &.open {
    width: 250px;
    .logo {
      &-name {
        opacity: 1;
        display: flex;
        align-items: center;
      }
    }
  }
  &.close {
    .logo {
      &-name {
        display: none;
      }
    }
  }
}
</style>
