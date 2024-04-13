<template>
  <div class="relative">
    <Titlebar />
    <div class="absolute relative top-10">
      <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    </div>
  </div>
</template>

<script setup>
import "~/assets/scss/main.scss";
import { appWindow } from "@tauri-apps/api/window";

onMounted(() => {
  setTimeout(() => {
    appWindow.show();
  }, 50);
});
</script>
