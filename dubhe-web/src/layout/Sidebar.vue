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
      <div id="my-scroll" style="margin: 6px 14px 0 14px">
        <ul class="nav-list" style="overflow: visible">
          <li
            v-for="(menuItem, index) in menuItems"
            :key="index"
            :id="'links_' + index"
          >
            <router-link
              :to="menuItem.link"
              v-tooltip="{ value: menuItem.name, disabled: isOpened }"
            >
              <i class="pi" :class="menuItem.icon || 'bx-square-rounded'" />
              <span class="links_name">{{ menuItem.name }}</span>
            </router-link>
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

let menuItems: Array<MenuItems> = [
  { name: "Devices", link: "/", icon: "pi-server", tooltip: "" },
];

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

<style lang="less">
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
    .nav-list {
      padding-inline-start: 0px;
    }
    li {
      position: relative;
      margin: 8px 0;
      list-style: none;
    }
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

  &.open {
    width: 250px;
    .logo {
      &-name {
        opacity: 1;
        display: flex;
        align-items: center;
      }
    }
    .item {
      .tooltip {
        display: none;
      }
    }
  }

  &.close {
    .logo {
      &-name {
        display: none;
      }
    }
    .item {
      span {
        display: none;
      }
    }
  }
}
</style>
