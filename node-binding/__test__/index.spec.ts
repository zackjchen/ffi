import test from 'ava'

import { add,  Algo} from '../index'

test('sync function from native code', (t) => {
  t.is(add(1,2), 3)
})

test('Algo failed', (t) => {
  var algo = new Algo("test")
  console.log(algo.hash("test"))
  t.is(algo.getName(), "Default")
})
