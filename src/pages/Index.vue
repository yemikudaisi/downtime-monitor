<template>
  <q-page>
    <div class="">
        <q-toolbar class="q-gutter-md bg-primary text-white">
          <div class="q-gutter-x-md">
            <q-btn round disable size="sm" icon="ion-pause" />
            <q-btn round disable size="sm" icon="ion-trash" />
          </div>
          <q-select color="grey-3" dense label-color="white" v-model="orderBy" :options="orderByOptions" label="Order by" style="width: 300px;">
            <template v-slot:append>
              <q-icon name="event" color="white" />
            </template>
          </q-select>
          <q-space />
          <q-btn size="md" dense label="Add new" icon="ion-add-circle" @click="showWebsiteDialog = true" />
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
        title="Websites"
        :data="websites"
        :columns="tableColumns"
        row-key="name"
      />
    </div>
    <q-dialog v-model="showWebsiteDialog" persistent transition-show="scale" transition-hide="scale">
      <q-card class="bg-primary text-white" style="width: 400px">
        <q-card-section>
          <div class="text-h6">New Website</div>
        </q-card-section>

        <q-card-section class="q-pt-none">
          <q-input square class='text-white' standout v-model="newWebsite.name" label="Name" />
          <q-input square standout v-model="newWebsite.url" label="URL" />
          <q-input square standout v-model="newWebsite.type" label="Monitor type" />
        </q-card-section>

        <q-card-actions align="right" class="bg-white text-teal">
          <q-btn flat label="OK" v-close-popup />
        </q-card-actions>
      </q-card>
    </q-dialog>
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
      websites: [],
      showWebsiteDialog: false,
      newWebsite: website
    }
  }
}

var website = {
  name: 'Car',
  url: 'url',
  type: 't',
  uptime: 0
}
</script>

<style scoped>
.order-select{
  min-width: 300px;
}
</style>
