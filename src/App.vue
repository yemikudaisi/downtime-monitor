<template>
  <div id="q-app">
    <router-view />
  </div>
</template>
<script>
import store from './store'

export default {
  name: 'App',
  store
}
</script>
