const isUp = require('is-up')

async function checkIsUp (url) {
  return await isUp(url)
}

export { checkIsUp }
