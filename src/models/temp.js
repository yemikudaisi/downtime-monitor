const { Model } = require('objection')

class MonitorEntry extends Model {
  static get tableName () {
    return 'persons'
  }

  static get idColumn () {
    return 'url'
  }

  static get jsonSchema () {
    return {
      type: 'object',
      required: ['url', 'name'],

      properties: {
        url: { type: 'string' },
        name: { type: ['string', 'null'] },
        type: { type: 'string', minLength: 1, maxLength: 255 },
        online: { type: 'string', minLength: 1, maxLength: 255 },
        lastChecked: { type: 'number' }
      }
    }
  }
}

export default MonitorEntry
