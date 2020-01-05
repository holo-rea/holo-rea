const {
  getDNA,
  buildConfig,
  buildRunner,
  buildGraphQL,
} = require('../init')

const runner = buildRunner()

const config = buildConfig({
  specification: getDNA('specification'),
}, {})

const exampleEntry = {
  label: 'kilgrams',
  symbol: 'kig',
}
const updatedExampleEntry = {
  label: 'kilograms',
  symbol: 'kg',
}

runner.registerScenario('Unit record API', async (s, t) => {
  const { alice } = await s.players({ alice: config }, true)
  alice.graphQL = buildGraphQL(alice)

  let createResp = await alice.graphQL(`
    mutation($rs: UnitCreateParams!) {
      res: createUnit(unit: $rs) {
        unit {
          id
        }
      }
    }
    `, {
    rs: exampleEntry,
  })
  await s.consistency()

  t.ok(createResp.data.res.unit.id, 'record created')
  let uId = createResp.data.res.unit.id

  const getResp = await alice.graphQL(`
    query($id: ID!) {
      res: unit(id: $id) {
        id
        label
        symbol
      }
    }
    `, {
    id: uId,
  })

  t.deepEqual(getResp.data.res, { 'id': uId, ...exampleEntry }, 'record read OK')

  const updateResp = await alice.graphQL(`
    mutation($rs: UnitUpdateParams!) {
      res: updateUnit(unit: $rs) {
        unit {
          id
        }
      }
    }
    `, {
    rs: { id: uId, ...updatedExampleEntry },
  })
  await s.consistency()

  t.notEqual(updateResp.data.res.unit.id, uId, 'update operation succeeded')
  t.equal(updateResp.data.res.unit.id, updatedExampleEntry.symbol, 'record index updated')
  uId = updateResp.data.res.unit.id

  // now we fetch the Entry again to check that the update was successful
  const updatedGetResp = await alice.graphQL(`
    query($id: ID!) {
      res: unit(id: $id) {
        id
        label
        symbol
      }
    }
  `, {
    id: uId,
  })

  t.deepEqual(updatedGetResp.data.res, { id: uId, ...updatedExampleEntry }, 'record updated OK')

  const deleteResult = await alice.graphQL(`
    mutation($id: String!) {
      res: deleteUnit(id: $id)
    }
  `, {
    id: uId,
  })
  await s.consistency()

  t.equal(deleteResult.data.res, true)

  const queryForDeleted = await alice.graphQL(`
    query($id: ID!) {
      res: unit(id: $id) {
        id
        label
        symbol
      }
    }
  `, {
    id: uId,
  })

  t.equal(queryForDeleted.errors.length, 1, 'querying deleted record is an error')
  t.notEqual(-1, queryForDeleted.errors[0].message.indexOf('No entry at this address'), 'correct error reported')
})

runner.run()
