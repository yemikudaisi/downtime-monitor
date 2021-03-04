<template>
  <q-page class="">
    <div class="q-pa-md">
      <div class="row no-wrap shadow-1">
        <q-toolbar class="col-8 bg-grey-3">
          <q-btn-dropdown stretch flat label="Dropdown">
            <q-list>
              <q-item>By Status</q-item>
            </q-list>
          </q-btn-dropdown>
          <q-separator dark vertical />
          <q-btn stretch flat label="Link" />
          <q-separator dark vertical />
          <q-btn stretch flat label="Link" />
          <q-space />
          <q-btn stretch flat label="Link" />
        </q-toolbar>
        <q-toolbar class="col-4 bg-primary text-white">
          <q-space />
          <q-btn flat round dense icon="bluetooth" class="q-mr-sm" />
          <q-btn flat round dense icon="more_vert" />
        </q-toolbar>
      </div>
    </div>
  </q-page>
</template>

<script>
export default {
  name: 'PageIndex'
}
</script>
