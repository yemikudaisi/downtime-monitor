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
const MonitorTypes = Object.freeze({ PING: 'PING', HTTPS: 'HTTPS' })
const MonitorStates = Object.freeze({ ACTIVE: 'ACTIVE', SUSPEND: 'SUSPEND' })

class OnlineMonitor {
  constructor (interval) {
    this.targets = new Map()
    this.pauseState = false
    this.monitorCallback = null
    this.timer = null
    this.pausedTargets = new Map()
  }

  addWebsite (url, monitorType) {
    this.targets[url] = monitorType
  }

  suspendAll () {
    this.pauseState = true
  }

  suspendOne (url) {
    try {
      this.pausedTargets[url] = true
    } catch (err) {
      console.log(err)
    }
  }

  start (callbackFunc) {
    this.monitorCallback = callbackFunc
    this.timer = setInterval(this.loop, this.interval)
  }

  stop () {
    if (this.timer)
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

  static checkWebsiteHttps (url, checkCallback) {
    internetAvailable({
      timeout: 5000, // maximum execution time
      retries: 2 // fail after five attempts
    }).then(() => {
      console.log("Internet available");
      httpCheckOnline(url, (r) => {
        console.log(url)
        checkCallback({ internet: true, url: url, type: MonitorTypes.HTTPS, online: r })
      })
    }).catch(() => {
      console.log("No internet");
      checkCallback({ internet: false})
    })
  }

  static checkWebsitePing (url, checkCallback) {
    return `${url} ${checkCallback}`
  }

  loop () {
    this.targets.forEach((value, key) => {
      if (value === MonitorTypes.HTTPS) {
        OnlineMonitor.checkWebsiteHttps(key, this.monitorCallback)
      } else if (value === MonitorTypes.PING) {
        OnlineMonitor.checkWebsitePing(key, this.monitorCallback)
      }
    })
  }
}

export { checkIsUp, httpCheckOnline, OnlineMonitor, MonitorTypes, MonitorStates }