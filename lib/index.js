const addon = require('../native/index.node');
const {makeDestination} = require('@statechannels/wallet-core')
const {hexZeroPad, defaultAbiCoder} = require('ethers').utils
const _ = require('lodash')

const iter = _.range(1000);

const outcome = [
  {
    amount: hexZeroPad('0x123', 32),
    destination: makeDestination('0xA5C9d076B3FC5910d67b073CBF75C4e13a5AC6E5')
  },
  {
    amount: hexZeroPad('0x5', 32),
    destination: makeDestination('0xBAF5D86514365D487ea69B7D7c85913E5dF51648')
  }
];

function encodeByEthers(outcome) {
  return defaultAbiCoder.encode(
    ['tuple(bytes32 destination, uint256 amount)[]'],
    [outcome.map(i => [i.destination, i.amount])]
  );
}


console.log(`Encoded by rust: ${addon.encodeOutcome(JSON.stringify(outcome)).join('')}`);
console.log(`Encoded by ethers: ${encodeByEthers(outcome)}`);

console.time('fast encode outcome')
iter.map(() => {
  addon.encodeOutcome(JSON.stringify(outcome))
})
console.timeEnd('fast encode outcome')

console.time('ethers encode outcome')
iter.map(() => encodeByEthers(outcome)
)
console.timeEnd('ethers encode outcome')

module.exports = addon;

console.time('data transfer');
iter.map(() => addon.logSerializedOutcome(JSON.stringify(outcome)))
console.timeEnd('data transfer')