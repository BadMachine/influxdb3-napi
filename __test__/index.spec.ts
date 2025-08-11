import test from 'ava'
import 'dotenv/config'

import { InfluxDbClient, Point } from '../index'

test('Test sql query from cloud serverless', async (t) => {
  const client = new InfluxDbClient(process.env.SERVER_URL || '', process.env.API_TOKEN || '')

  const result = client.query({
    database: 'test',
    query: 'SELECT * FROM "tag_keys"',
  })

  const arr = []
  for await (const item of result) {
    arr.push(item)
  }

  t.true(arr.length > 0)
})

test('Test write lp to cloud serverless', async (t) => {
  const client = new InfluxDbClient(process.env.SERVER_URL || '', process.env.API_TOKEN || '')

  const point = Point.fromMeasurement('tag_keys')

  const PLAIN = 'plain'
  const WITH_SPACE = 'with space'
  const WITH_COMMA = 'with,comma'
  const WITH_EQ = 'with=eq'
  const WITH_DOUBLE_QUOTE = 'with"doublequote"'
  const WITH_SINGLE_QUOTE = "with'singlequote"
  const WITH_BACKSLASH = `with\ backslash`

  point.setTag(PLAIN, 'dummy')
  point.setTag(WITH_SPACE, 'dummy')
  point.setTag(WITH_COMMA, 'dummy')
  point.setTag(WITH_EQ, 'dummy')
  point.setTag(WITH_DOUBLE_QUOTE, 'dummy')
  point.setTag(WITH_SINGLE_QUOTE, 'dummy')
  point.setTag(WITH_BACKSLASH, 'dummy')
  point.setBooleanField('dummy', true)

  // write_options.no_sync = Some(false);
  // write_options.precision = Some(Precision::V2(TimeUnitV2::Nanosecond));

  try {
    const lp = point.toLineProtocol("ns") || '';
    console.log(lp)
    await client.write([lp], "test",  {
      noSync: false,
      precision: { type: 'V2', field0: 'ns' },
      gzip: false,
    });
    t.pass()
  } catch (e) {
    console.error(e);
    console.log('qweqwe ', e)
    t.fail()
  }
})
