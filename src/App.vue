<template>
  <div id="q-app">
    <router-view />
  </div>
</template>
<script>
import store from './store'
import { createSchema } from './helpers/dbUtils'

export default {
  name: 'App',
  store,
  created: function () {
    console.log('vm created')
    // console.log(`KNEX POOL -> free: ${getPool().numUsed()}, used: ${getPool().numFree()}`)
    createSchema()
      .then(r => {})
      .catch(e => {
        console.error('DB error')
        console.error(e)
      })
  }
}
</script>
