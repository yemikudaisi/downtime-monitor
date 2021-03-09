const { Model } = require('objection')

class MonitorEntry extends Model {
  static get tableName () {
    return 'monitor_entry'
  }

  static get idColumn () {
    return 'id'
  }

  $beforeInsert () {
    this.created_at = new Date().toISOString()
  }

  $beforeUpdate () {
    this.updated_at = new Date().toISOString()
  }

  static get jsonSchema () {
    return {
      type: 'object',
      required: ['url', 'name'],

      properties: {
        id: { type: 'integer' },
        url: { type: 'string' },
        name: { type: 'string' },
        type: { type: 'string', minLength: 1, maxLength: 255 },
        updated_at: { type: 'date' }
      }
    }
  }
}

export default MonitorEntry
