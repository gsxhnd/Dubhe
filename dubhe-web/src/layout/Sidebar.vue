<template>
  <div class="sidebar" :class="isOpened ? 'open' : ''">
    <div class="sidebar-logo">
      <template v-if="menuLogo">
        <img />
      </template>
      <template v-else>
        <i class="bx icon"></i>
      </template>
      <div class="logo-name">{{ menuTitle }}</div>
    </div>
    <div class="sidebar-item">
      <div id="my-scroll" style="margin: 6px 14px 0 14px">
        <ul class="nav-list" style="overflow: visible">
          <li
            v-for="(menuItem, index) in menuItems"
            :key="index"
            :id="'links_' + index"
          >
            <router-link v-if="isUsedVueRouter" :to="menuItem.link">
              <i class="bx" :class="menuItem.icon || 'bx-square-rounded'" />
              <span class="links_name">{{ menuItem.name }}</span>
            </router-link>
            <a
              v-else
              @click.stop.prevent="$emit('menuItemClcked', menuItem.link)"
              :href="menuItem.link"
            >
              <i class="bx" :class="menuItem.icon || 'bx-square-rounded'" />
              <span class="links_name">{{ menuItem.name }}</span>
            </a>
            <span :data-target="'links_' + index" class="tooltip">{{
              menuItem.tooltip || menuItem.name
            }}</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface MenuItems {
  name: string;
  link: string;
  icon: string;
  tooltip: string;
}

defineProps({
  // searchPlaceholder: { type: String },
  // searchTooltip: { type: String },
  // isSearch: { type: Boolean, default: false }
  menuLogo: { type: String, default: "" },
  menuTitle: { type: String, default: "Logo Name" },
  isOpened: { type: Boolean, default: true },
  isUsedVueRouter: { type: Boolean, default: false },
  menuItems: Array<MenuItems>,
  bgColor: {
    type: String,
    default: "#11101d",
  },
});
</script>

<style lang="less">
@bg-color: #11101d;
@icons-color: #fff;
@items-tooltip-color: #e4e9f7;

.sidebar {
  position: relative;
  display: flex;
  flex-direction: column;
  position: fixed;
  left: 0;
  top: 0;
  height: 100%;
  min-height: min-content;
  /* overflow-y: auto; */
  width: 78px;
  background: @bg-color;
  /* padding: 6px 14px 0 14px; */
  z-index: 99;
  transition: all 0.5s ease;

  &.open {
    width: 250px;
  }
  &-logo {
    margin: 6px 14px 0 14px;
  }
  &-item {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    flex-grow: 1;
    max-height: calc(100% - 60px);
    li {
      position: relative;
      margin: 8px 0;
      list-style: none;
    }
    i {
      position: relative;
      margin: 8px 0;
      list-style: none;
      color: #fff;
      height: 60px;
      min-width: 50px;
      font-size: 28px;
      text-align: center;
      line-height: 60px;
      a {
        display: flex;
        height: 100%;
        width: 100%;
        border-radius: 12px;
        align-items: center;
        text-decoration: none;
        transition: all 0.4s ease;
        background: @bg-color;
        &:hover {
          background: #fff;
        }
      }
    }
  }
}
</style>
