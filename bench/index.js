const hash = require('./hash')
const sign = require('./sign')
const recover = require('./recover')

const run = async () => {
  await hash()
  await sign()
  await recover()
}

run()
