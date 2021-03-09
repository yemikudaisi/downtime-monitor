<template>
  <q-page>
    <div class="bg-primary">
      <q-toolbar class="q-gutter-md bg-primary text-white" style="margin-right: 100px !important;">
        <div class="q-gutter-x-md">
          <q-btn @click="editWebsite" round :disable="disableEditAction" size="sm" icon="ion-create" />
          <q-btn round :disable="disableEntryAction" size="sm" icon="ion-pause" />
          <q-btn round :disable="disableEntryAction" size="sm" icon="ion-trash" />
          <q-btn round @click="updateAllOnlineStatus" size="sm" icon="ion-refresh" />
        </div>
        <q-select color="black" dense label-color="white" v-model="selectedTableMode" :options="tableModeOptions" label="Table Mode" style="width: 300px;">
          <template v-slot:append>
            <q-icon name="ion-grid" color="white" />
          </template>
        </q-select>
        <q-space />
        <q-btn size="md" dense label="Add new" icon="ion-add-circle" @click="addWebsite" />
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
        :loading="loading"
        ref="websitesTable"
        title="Websites"
        :data="websites"
        :columns="tableColumns"
        row-key="name"
        :selected-rows-label="getSelectedString"
        selection="multiple"
        :selected="selectedWebsites"
        @selection="onWebsiteSelection"
        color="primary"
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
        <template v-slot:body-cell-updated_at="props">
            <q-td :props="props">
              {{getFriendlyDate(props.row.updated_at)}}
            </q-td>
        </template>
        <template v-slot:body-cell-created_at="props">
            <q-td :props="props">
              {{getFriendlyDate(props.row.updated_at)}}
            </q-td>
        </template>
      </q-table>
    </div>
    <q-dialog v-model="showWebsiteDialog" persistent transition-show="scale" transition-hide="scale">
      <q-card style="width: 400px">
        <q-card-section class="bg-primary text-white">
          <div class="text-h6">New Website</div>
        </q-card-section>

        <q-card-section class="q-pt-none">
          <q-input square class='text-white' v-model="newWebsite.name" label="Name" />
          <q-input square v-model="newWebsite.url" label="URL" />
          <q-select square v-model="newWebsite.type" :options="monitorTypeOptions" label="Monitor type" style="width: 300px;" />
        </q-card-section>

        <q-card-actions align="right" class="text-white bg-primary">
          <div class="q-gutter-md">
            <q-btn flat label="OK" v-close-popup @click="saveOrUpdateWebsite" />
            <q-btn flat label="CANCEL" v-close-popup />
          </div>
        </q-card-actions>
      </q-card>
    </q-dialog>
  </q-page>
</template>

<script>
import { getEntries, addEntry, updateEntry } from '../helpers/dbUtils'
import { httpCheckOnline } from '../helpers/monitorUtils'
require('datejs')
const { Notification } = require('electron')

export default {
  name: 'PageIndex',
  data () {
    return {
      searchText: '456',
      selectedTableMode: 'Table',
      tableGridMode: false,
      tableModeOptions: [
        'Table', 'Grid'
      ],
      editMode: false,
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
        { name: 'updated_at', align: 'center', label: 'Updated', field: 'updated_at', sortable: true },
        { name: 'created_at', align: 'center', label: 'Created', field: 'created_at', sortable: true },
        { name: 'status', align: 'center', label: 'Status', field: 'online', sortable: true }
      ],
      websites: [],
      selectedWebsites: [],
      lastIndex: null,
      showWebsiteDialog: false,
      loading: false,
      newWebsite: website,
      monitorTypeOptions: ['HTTPS', 'PING'],
      monitorTimer: null,
      timerInterval: 60000 // In miliseconds (5 seconds)
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
  computed: {
    disableEntryAction () {
      return this.selectedWebsites.length <= 0
    },
    disableEditAction () {
      return this.selectedWebsites.length !== 1
    }
  },
  methods: {
    showInternetNotification () {
      this.$q.notify({
        spinner: true,
        message: 'No internet.',
        timeout: 5000
      })
    },
    getFriendlyDate: function (datetime) {
      return new Date(datetime).toString('d MMM yyyy')
    },
    saveOrUpdateWebsite: function () {
      var obj = this
      if (this.editMode) {
        updateEntry(this.newWebsite)
      } else {
        addEntry(this.newWebsite)
          .then(() => {
            console.log('added')
            obj.loadng = false
            this.updateEntryList()
          })
          .catch((e) => {
            console.log(e)
            if (e.name === 'UniqueViolationError') {
              obj.$q.notify({ message: 'Website URL must be unique', color: 'orange' })
            }
            obj.$q.notify({ message: 'Entry addition entry failed', color: 'orange' })
            obj.loadng = false
          })
      }
    },
    editWebsite: function () {
      this.editMode = true
      this.newWebsite = this.selectedWebsites[0]
      this.showWebsiteDialog = true
    },
    addWebsite: function () {
      this.editMode = false
      this.newWebsite = { ...website }
      this.showWebsiteDialog = true
    },
    getSelectedString () {
      return this.selectedWebsites.length === 0 ? '' : `${this.selectedWebsites.length} record${this.selectedWebsites.length > 1 ? 's' : ''} selected of ${this.websites.length}`
    },
    onWebsiteSelection ({ rows, added, evt }) {
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
    },
    updateAllOnlineStatus () {
      var obj = this
      // Check if each website entry is online and update status
      setTimeout(function () {
        obj.websites.forEach((item, idx, arr) => {
          httpCheckOnline(item.url, (r) => {
            console.log(r)
            item.online = r
            if (!r) {
              const n = new Notification('Website Offline', {
                body: `${item.url} is currently offline`
              })
              n.show()
            }
          })
        })
      }, 3000)
    },
    updateEntryList () {
      var obj = this
      this.loading = true
      getEntries()
        .then(entries => {
          obj.websites = entries.map(item => {
            item.online = false
            return item
          })
          obj.loading = false
          obj.updateAllOnlineStatus()
        })
        .catch(e => { console.log(e) })
    }
  },
  mounted () {
    this.$nextTick(function () {})
    this.updateEntryList()
  }
}

var website = {
  name: '',
  url: '',
  type: '',
  updated_at: null,
  created_at: null
}
</script>

<style scoped>
</style>
