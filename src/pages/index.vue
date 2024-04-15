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
    <div class="flex flex-wrap mt-5 h-full gap-10 justify-center">
      <StatusCard v-for="item in home.items" :service="item" />
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
</script>

<style lang="scss"></style>
