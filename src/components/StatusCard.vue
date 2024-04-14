<template>
  <div class="server-status-card">
    <div class="flex flex-col">
      <div class="flex justify-between">
        <div class="text-2xl pb-4 txt-color-1">{{ service.name }}</div>
        <div>
          <div class="status-indicator" :class="getStatusColor(service.status)"></div>
        </div>
      </div>
      <div class="txt-color-2 leading-loose">{{ (service.status == ServiceStatus.Online? 'Online': 'Offline') }}, {{ statusAgo }}</div>
      <div class="txt-color-2 leading-loose">Last checked {{ checkedAgo }}</div>
      <VueUiSparkHistogram :dataset="dataset" :config="config" class="w-86" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ServiceOverview, ServiceStatus } from '~/types';
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime';
import { type VueUiSparkHistogramDatasetItem, VueUiSparkHistogram } from "vue-data-ui";
import { config } from './mini-historam-options';

dayjs.extend(relativeTime);

const home = defineProps<{
  service: ServiceOverview,
}>()

const checkedAgo = computed(() => {
  return dayjs().to(dayjs(home.service.lastChecked))
});

const statusAgo = computed(() => {
  return dayjs().to(dayjs(home.service.statusDuration))
});

function getStatusColor(status: ServiceStatus) {
  switch (status) {
    case ServiceStatus.Offline:
      return 'bg-danger'
    case ServiceStatus.Online:
      return 'bg-primary'
  }
}

const dataset: VueUiSparkHistogramDatasetItem[] = [
    {
        value: 1.2,
        valueLabel: "20%",
        timeLabel: "09:00",
        intensity: 0.2,
    },
    {
        value: 1.3,
        valueLabel: "50%",
        timeLabel: "10:00",
        intensity: 0.5,

    },
    {
        value: 1.1,
        valueLabel: "60%",
        timeLabel: "11:00",
        intensity: 0.6,

    },
    {
        value: 0.8,
        valueLabel: "70%",
        timeLabel: "12:00",
        intensity: 0.7,

    },
    {
        value: 2,
        valueLabel: "100%",
        timeLabel: "13:00",
        intensity: 1,

    },
    {
        value: 2.1,
        valueLabel: "100%",
        timeLabel: "14:00",
        intensity: 1,

    },
    {
        value: 2.3,
        valueLabel: "80%",
        timeLabel: "15:00",
        intensity: 0.8,

    },
    {
        value: 2.1,
        valueLabel: "70%",
        timeLabel: "16:00",
        intensity: 0.7,

    },
    {
        value: 0.9,
        valueLabel: "60%",
        timeLabel: "17:00",
        intensity: 0.6,

    },
    {
        value: 0.7,
        valueLabel: "50%",
        timeLabel: "18:00",
        intensity: 0.5,

    },
    {
        value: 0.3,
        valueLabel: "30%",
        timeLabel: "19:00",
        intensity: 0.3,

    },
    {
        value: 0.2,
        valueLabel: "20%",
        timeLabel: "20:00",
        intensity: 0.2,

    },
]
</script>