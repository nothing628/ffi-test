import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import viteWasm from "vite-plugin-wasm";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), viteWasm()],
});
