import { defineConfig } from "vite";
import { splitVendorChunkPlugin } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";

// https://vitejs.dev/config/
export default defineConfig({
  // root: path.join(__dirname, "web"),
  // publicDir: path.join(__dirname, "web/public"),
  base: "/",
  plugins: [vue(), splitVendorChunkPlugin()],
  server: {
    host: "0.0.0.0",
    port: 3000,
    strictPort: true,
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  css: {
    preprocessorOptions: {
      less: {
        modifyVars: {
          hack: `true; @import (reference) "${path.resolve(
            "src/style/base.less"
          )}";`,
        },
      },
    },
  },
  build: {
    cssCodeSplit: true,
  },
});
