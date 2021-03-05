<template>
  <q-page>
    <div class="bg-primary">
      <q-toolbar class="q-gutter-md bg-primary text-white" style="margin-right: 100px !important;">
        <div class="q-gutter-x-md">
          <q-btn round disable size="sm" icon="ion-pause" />
          <q-btn round disable size="sm" icon="ion-trash" />
        </div>
        <q-select color="grey-3" dense label-color="white" v-model="orderBy" :options="orderByOptions" label="Order by" style="width: 300px;">
          <template v-slot:append>
            <q-icon name="ion-reorder" color="white" />
          </template>
        </q-select>
        <q-select color="black" dense label-color="white" v-model="selectedTableMode" :options="tableModeOptions" label="Table Mode" style="width: 300px;">
          <template v-slot:append>
            <q-icon name="ion-grid" color="white" />
          </template>
        </q-select>
        <q-space />
        <q-btn size="md" dense label="Add new" icon="ion-add-circle" @click="showWebsiteDialog = true" />
      </q-toolbar>
    </div>
    <q-separator />
    <q-toolbar class="row q-my-md">
      <q-input bottom-slots v-model="searchText" class="col-12" label="Enter website URL" stretch :dense="true">
        <template v-slot:hint>
          enter fully qualified name for website
        </template>
        <template v-slot:append>
          <q-btn round dense flat icon="search" />
        </template>
      </q-input>
    </q-toolbar>
    <div class="q-pa-md">
      <q-table
        :grid="tableGridMode ? true : false"
        ref="websitesTable"
        title="Websites"
        :data="websites"
        :columns="tableColumns"
        row-key="name"
        :selected-rows-label="getSelectedString"
        selection="multiple"
        :selected="selectedWebsites"
        @selection="onWebsiteSelection"
      >
        <template v-slot:body-cell-name="props">
          <q-td :props="props">
            <div class="my-table-details">
              {{ props.row.name }}
            </div>
            <q-badge color="primary" :label="props.row.url" />
          </q-td>
      </template>
      <template v-slot:body-cell-status="props">
          <q-td :props="props">
            <q-icon name="ion-radio-button-on" v-bind:color="getStatusColor(props.row.online)" style="font-size: 2em;" />
          </q-td>
      </template>
      </q-table>
    </div>
    <div class="q-mt-md">
      Selected: {{ JSON.stringify(selectedWebsites) }}
    </div>
    <q-dialog v-model="showWebsiteDialog" persistent transition-show="scale" transition-hide="scale">
      <q-card style="width: 400px">
        <q-card-section class="bg-primary text-white">
          <div class="text-h6">New Website</div>
        </q-card-section>

        <q-card-section class="q-pt-none">
          <q-input square class='text-white' v-model="newWebsite.name" label="Name" />
          <q-input square v-model="newWebsite.url" label="URL" />
          <q-input square v-model="newWebsite.type" label="Monitor type" />
        </q-card-section>

        <q-card-actions align="right" class="bg-white text-teal">
          <q-btn flat label="OK" v-close-popup @click="addNewWebsite" />
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
      selectedTableMode: 'Table',
      tableGridMode: false,
      tableModeOptions: [
        'Table', 'Grid'
      ],
      tableColumns: [
        {
          name: 'name',
          required: true,
          label: 'Site Name',
          align: 'left',
          field: row => row.name,
          format: val => `${val}`,
          sortable: true
        },
        { name: 'type', label: 'Type', field: 'type', sortable: true },
        { name: 'status', align: 'center', label: 'Status', field: 'online', sortable: true }
      ],
      websites: [],
      showWebsiteDialog: false,
      newWebsite: website,
      selectedWebsites: [],
      lastIndex: null
    }
  },
  watch: {
    selectedTableMode: function (val) {
      if (val === 'Grid') {
        this.tableGridMode = true
        return
      }
      this.tableGridMode = false
    }
  },
  methods: {
    addNewWebsite: function () {
      console.log(this.newWebsite)
      this.websites.push(this.newWebsite)
      this.newWebsite = { ...website }
    },
    getSelectedString () {
      return this.selectedWebsites.length === 0 ? '' : `${this.selectedWebsites.length} record${this.selectedWebsites.length > 1 ? 's' : ''} selected of ${this.websites.length}`
    },
    onWebsiteSelection ({ rows, added, evt }) {
      console.log(this.$refs.websitesTable)
      if (rows.length === 0 || this.$refs.websitesTable === undefined) {
        return
      }

      const row = rows[0]
      const filteredSortedRows = this.$refs.websitesTable.filteredSortedRows
      const rowIndex = filteredSortedRows.indexOf(row)
      const lastIndex = this.lastIndex

      this.lastIndex = rowIndex
      document.getSelection().removeAllRanges()

      if (this.$q.platform.is.mobile === true) {
        evt = { ctrlKey: true }
      } else if (evt !== Object(evt) || (evt.shiftKey !== true && evt.ctrlKey !== true)) {
        this.selectedWebsites = added === true ? rows : []

        return
      }

      const operateSelection = added === true
        ? selRow => {
          const selectedIndex = this.selectedWebsites.indexOf(selRow)
          if (selectedIndex === -1) {
            this.selectedWebsites = this.selectedWebsites.concat(selRow)
          }
        }
        : selRow => {
          const selectedIndex = this.selectedWebsites.indexOf(selRow)
          if (selectedIndex > -1) {
            this.selectedWebsites = this.selectedWebsites.slice(0, selectedIndex).concat(this.selectedWebsites.slice(selectedIndex + 1))
          }
        }

      if (lastIndex === null || evt.shiftKey !== true) {
        operateSelection(row)
        return
      }

      const from = lastIndex < rowIndex ? lastIndex : rowIndex
      const to = lastIndex < rowIndex ? rowIndex : lastIndex
      for (let i = from; i <= to; i += 1) {
        operateSelection(filteredSortedRows[i])
      }
    },
    getStatusColor (val) {
      if (val) {
        return 'green'
      }
      return 'red'
    }
  }
}

var website = {
  name: 'Army Website',
  url: 'army.mil.ng',
  type: 'PING',
  online: true
}
</script>

<style scoped>
.order-select{
  min-width: 300px;
}
</style>
