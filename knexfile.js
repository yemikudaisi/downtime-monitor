import { remote } from 'electron'
const path = require('path')

module.exports = {
  development: {
    client: 'sqlite3',
    connection: {
      filename: './dev.sqlite3'
    },
    useNullAsDefault: true,
    // Getting SQlite to honour contraints
    pool: {
      afterCreate: (conn, cb) =>
        conn.run('PRAGMA foreign_keys = ON', cb)
    }
  },

  test: {
    client: 'sqlite3',
    connection: {
      filename: ':memory:'
    },
    useNullAsDefault: true,
    seeds: {
      directory: path.join(__dirname, 'tests/seeds')
    },
    migrations: {
      directory: path.join(__dirname, 'migrations')
    }
  },

  production: {
    client: 'sqlite3',
    connection: process.env.DATABASE_URL,
    pool: {
      min: 2,
      max: 10
    },
    migrations: {
      directory: path.join(__dirname, './server/db/migrations')
    },
    seeds: {
      directory: path.join(__dirname, './seeds')
    }
  }
}
