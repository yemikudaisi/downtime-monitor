<template>
  <q-page>
    <div class="">
        <q-toolbar class="q-gutter-md bg-primary text-white">
          <div class="q-gutter-md">
            <q-btn round disable size="sm" icon="ion-pause" />
            <q-btn round disable size="sm" icon="ion-trash" />
          </div>
          <q-select v-model="orderBy" :options="orderByOptions" label="Order by" style="width: 300px;" />
          <q-space />
          <q-btn size="md" dense label="Add new" icon="ion-add-circle" />
        </q-toolbar>
      <q-separator />
      <div class="row no-wrap">
        <q-toolbar class="col-12 row">
          <q-input bottom-slots v-model="searchText" class="col-md-12" label="Enter website URL" stretch :dense="true">
            <template v-slot:hint>
              enter fully qualified name for website
            </template>

            <template v-slot:append>
              <q-btn round dense flat icon="search" />
            </template>
          </q-input>
        </q-toolbar>
      </div>
    </div>
    <div class="q-pa-md">
      <q-table
        title="Treats"
        :data="websites"
        :columns="tableColumns"
        row-key="name"
      />
    </div>
  </q-page>
</template>

<script>
export default {
  name: 'PageIndex',
  data () {
    return {
      searchText: '',
      isSelected: false,
      orderBy: null,
      orderByOptions: [
        'By type', 'By status'
      ],
      tableColumns: [
        {
          name: 'name',
          required: true,
          label: 'Site Name',
          align: 'left',
          field: row => row.url,
          format: val => `${val}`,
          sortable: true
        },
        { name: 'type', label: 'Type', field: 'type', sortable: true },
        { name: 'Status', label: 'Uptime', field: 'upTime', sortable: true }
      ],
      websites: []
    }
  }
}
</script>

<style scoped>
.order-select{
  min-width: 300px;
}
</style>
