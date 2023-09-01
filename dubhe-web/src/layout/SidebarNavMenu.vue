<template>
  <div class="siderbar-nav" style="margin: 6px 14px 0 14px">
    <ul class="nav-list" style="overflow: visible">
      <li
        class="nav-item"
        v-for="(menuItem, index) in menuItems"
        :key="index"
        :id="'links_' + index"
      >
        <router-link
          class="link"
          :to="menuItem.link"
          v-tooltip="{
            value: menuItem.name,
            disabled: layoutStore.sidebar.open,
          }"
        >
          <i class="pi" :class="menuItem.icon || 'bx-square-rounded'" />
          <span
            class="link-name"
            :class="layoutStore.sidebar.open ? 'open' : 'close'"
            >{{ menuItem.name }}</span
          >
        </router-link>
      </li>
    </ul>
  </div>
</template>

<style lang="less" scoped>
.siderbar-nav {
  .nav-list {
    padding-inline-start: 0px;
    .nav-item {
      position: relative;
      margin: 8px 0;
      list-style: none;
      .link {
        i {
          position: relative;
          margin: 8px 0;
          list-style: none;
          color: aqua;
          height: 60px;
          min-width: 50px;
          font-size: 28px;
          text-align: center;
          line-height: 60px;
        }
        &-name {
          &.close {
            display: none;
          }
        }
      }
    }
  }
}
</style>

<script lang="ts" setup>
import { useLayoutStore } from "@/stores/layout";
const layoutStore = useLayoutStore();

interface MenuItems {
  name: string;
  link: string;
  icon: string;
  tooltip: string;
}

let menuItems: Array<MenuItems> = [
  { name: "Devices", link: "/", icon: "pi-server", tooltip: "" },
];
</script>
