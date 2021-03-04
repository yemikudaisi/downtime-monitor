<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          icon="ion-menu"
          aria-label="Menu"
          @click="leftDrawerOpen = !leftDrawerOpen"
        />

        <q-toolbar-title>
          Downtime Monitor
        </q-toolbar-title>

        <div>Version {{ $q.version }}</div>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
      content-class="bg-grey-1"
    >
      <q-list>
        <q-item-label
          header
          class="text-grey-8"
        >
          Menu
        </q-item-label>
        <DrawerLink
          v-for="link in essentialLinks"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import DrawerLink from 'src/components/DrawerLink.vue'

const linksData = [
  {
    title: 'Dashboard',
    caption: 'Overview',
    icon: 'ion-pie',
    link: '/'
  },
  {
    title: 'Settings',
    caption: 'Preferences',
    icon: 'ion-cog',
    link: '/settings'
  }
]

export default {
  name: 'MainLayout',
  components: { DrawerLink },
  data () {
    return {
      leftDrawerOpen: false,
      essentialLinks: linksData
    }
  }
}
</script>
