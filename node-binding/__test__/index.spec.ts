import test from 'ava'

import { Algo} from '../index'


test('Algo failed', (t) => {
  var algo = new Algo("test")
  console.log(algo.hash("test"))
  t.is(algo.getName(), "Default")
})
