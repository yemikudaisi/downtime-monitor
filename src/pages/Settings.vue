<template>
  <q-page >
    <h1>Options</h1>
    <div class="q-pa-md">
      <q-slider v-model="standard" :min="5" :max="50"/>
    </div>
  </q-page>
</template>

<script>
export default {
  name: 'PageIndex',
  data: function () {
    return {
      standard: 5
    }
  }
}
</script>
