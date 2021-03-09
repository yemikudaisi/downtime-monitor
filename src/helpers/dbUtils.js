// import { app } from 'electron'
import { remote } from 'electron'
const path = require('path')
import MonitorEntry from '../models/MonitorEntry'
// import Sequelize from 'sequelize'

const app = remote.app

const appDataDir = path.join(app.getPath('userData'), 'downtime-monitor')
const dbDir = path.join(appDataDir, '/db')

var fs = require('fs')

if (!fs.existsSync(appDataDir)) {
  console.log('creating app directories')
  fs.mkdirSync(appDataDir)
  fs.mkdirSync(dbDir)
}

/** const sequelize = new Sequelize({
  dialect: 'sqlite',
  storage: path.join(dbDir, 'database.sqlite')
});

(async () => {
  await sequelize.sync()
})() */

const { Model } = require('objection')
const Knex = require('knex')

// Initialize knex.
const knex = Knex({
  client: 'sqlite3',
  useNullAsDefault: true,
  connection: {
    filename: path.join(dbDir, 'database.sqlite')
  }
})

// Give the knex instance to objection.
Model.knex(knex)

async function getEntries () {
  return await MonitorEntry.query()
}

async function addEntry (entry) {
  console.log(entry.name)
  await MonitorEntry.query().insert(entry)
}

async function deleteEntry (url) {
  await MonitorEntry.query().delete().where({ url: url })
}

async function updateEntry (entry) {
  await MonitorEntry.query().patch(entry).where({ id: entry.id })
}

async function updateEntryField (id, field, value) {
  await MonitorEntry.query().patch({ field: 'value' }).where({ url: id })
}

async function createSchema () {
  if (await knex.schema.hasTable('monitor_entry')) {
    console.log('schema exist')
    return 'Schema exists'
  }

  // Create database schema. You should use knex migration files
  // to do this. We create it here for simplicity.
  await knex.schema.createTable('monitor_entry', table => {
    table.increments('id').primary()
    table.string('url').unique().notNullable()
    table.string('name').notNullable()
    table.string('type').notNullable()
    table.timestamp('updated_at')
    table.timestamp('created_at')
  }) // .toSQL().forEach(query => console.log(query.sql))
  console.log('schema created')
  return 'Schema created'
}

export { getEntries, addEntry, deleteEntry, updateEntry, updateEntryField, createSchema }
