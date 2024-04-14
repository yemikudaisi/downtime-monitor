<template>
  <div class="server-status-card">
    <div class="flex flex-col">
      <div class="flex justify-between">
        <div class="text-2xl pb-4 txt-color-1">{{ service.name }}</div>
        <div>
          <div class="status-indicator" :class="getStatusColor(service.status)"></div>
        </div>
      </div>
      <div class="txt-color-2 leading-loose">Online, {{ service.statusDuration }}</div>
      <div class="txt-color-2 leading-loose">Last checked {{ service.lastChecked }} ago</div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ServiceOverview, ServiceStatus } from '~/types';

defineProps<{
  service: ServiceOverview,
}>()

function getStatusColor(status: ServiceStatus) {
  switch (status) {
    case ServiceStatus.Offline:
      return 'bg-danger'
    case ServiceStatus.Online:
      return 'bg-primary'
  }
}
</script>