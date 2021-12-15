import { remote } from 'electron'
const path = require('path')
var fs = require('fs')

export const getAppDataDir = () => {
  const app = remote.app
  const dir = path.join(app.getPath('userData'), 'downtime-monitor')
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir)
  }
  return dir
}

export const getDbDir = () => {
  const dir = path.join(getAppDataDir(), '/db')
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir)
  }
  return dir
}
