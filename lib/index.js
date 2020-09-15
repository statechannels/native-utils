const addon = require('../native/index.node');
const {BN, makeDestination} = require('@statechannels/wallet-core')

const outcome = [
  {
    amount: BN.from('0x5'),
    destination: makeDestination('0xA5C9d076B3FC5910d67b073CBF75C4e13a5AC6E5')
  },
  {
    amount: BN.from('0x5'),
    destination: makeDestination('0xBAF5D86514365D487ea69B7D7c85913E5dF51648')
  }
];

console.log(addon.logOutcome(outcome));

module.exports = addon;