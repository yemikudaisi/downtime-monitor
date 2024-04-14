<template>
  <div class="flex-col">
    <div class="flex justify-between">
      <div>
        <span class="main-header text-2xl text-white">Service Status</span>
      </div>
      <div class="btn-primary">
        <Icon name="heroicons:plus-circle" class="btn-icon block" />
        Add Service
      </div>
    </div>
    <div class="flex flex-wrap mt-5 h-full md:gap-20 gap-40">
      <div v-for="item in home.items" class="server-status-card">
        <div class="flex flex-col">
          <div class="flex justify-between">
            <div class="text-2xl pb-4 txt-color-1">{{ item.name }}</div>
            <div>
              <div class="status-indicator" :class="getStatusColor(item.status)"></div>
            </div>
          </div>
          <div class="txt-color-2 leading-loose">Online, {{ item.statusDuration }}</div>
          <div class="txt-color-2 leading-loose">Last checked {{ item.lastChecked }} ago</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ServiceOverview, ServiceStatus } from '~/types';
import { db } from "~/helpers/db-mock";
interface HomeState {
  items: ServiceOverview[];

}

const home: HomeState = reactive(
  {
    items: db.services.overview,
  }
)

function getStatusColor(status: ServiceStatus) {
  switch (status) {
    case ServiceStatus.Offline:
      return 'bg-danger'
    case ServiceStatus.Online:
      return 'bg-primary'
  }

}
</script>

<style lang="scss"></style>
