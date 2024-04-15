<template>
  <div class="relative">
    <Titlebar />
    <div class="relative w-full top-10">
      <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    </div>
  </div>
</template>

<script setup>
import "~/assets/scss/main.scss";
import { appWindow } from "@tauri-apps/api/window";
import "vue-data-ui/style.css"
import { App } from "./core/App";

const app = new App();
app.start();

onMounted(() => {
  setTimeout(() => {
    appWindow.show();
  }, 50);
});
</script>
