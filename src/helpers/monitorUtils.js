const isUp = require('is-up')

// Check is a website is online using
async function checkIsUp (url) {
  return await isUp(url)
}

function httpCheckOnline (url, callbackFunc) {
  var http = require('https')
  http.globalAgent.options.rejectUnauthorized = false

  try {
    http.get({ host: url }, function (res) {
      console.log(url)
      console.log(res)
      if (res.statusCode === 200 || res.statusCode === 301) {
        callbackFunc(true)
      } else {
        callbackFunc(false)
      }
    })
  } catch (err) {
    console.log(err)
    callbackFunc(false)
  }
}
const MonitorType = Object.freeze({ PING: 'PING', HTTPS: 'HTTPS' })
const MonitorStates = Object.freeze({ ACTIVE: 'ACTIVE', SUSPEND: 'SUSPEND' })

class OnlineMonitor {
  constructor (websiteList, interval) {
    this.targets = websiteList
    this.pauseState = false
    this.monitorCallback = null
    this.timer = null
  }

  addWebsite (entry) {

  }

  suspendAll () {

  }

  suspendOne () {

  }

  start (callbackFunc) {
    this.monitorCallback = callbackFunc
    this.timer = setInterval(this.loop, this.interval)
  }

  stop () {
    clearInterval(this.timer)
  }

  restart () {
    this.stop()
    this.start()
  }

  changeInterval (newInterval) {
    this.interval = newInterval
    this.restart()
  }

  static updateOne (website, checkCallback) {
    httpCheckOnline(website.url, (r) => {
      console.log(r)
      checkCallback(website.url, r)
    })
  }

  loop () {
    this.targets.forEach(element => {
      if (element.state !== MonitorStates.SUSPEND) {
        OnlineMonitor.updateOne(element)
      }
    })
  }
}

export { checkIsUp, httpCheckOnline, OnlineMonitor, MonitorType }
