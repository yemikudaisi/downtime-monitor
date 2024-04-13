<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window';

onMounted(() => {
    setTimeout(() => {
        appWindow.show();
    }, 50);
})
</script>
