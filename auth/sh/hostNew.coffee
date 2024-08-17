#!/usr/bin/env coffee

> @3-/redis/R.js
  @3-/reverse
  @3-/dbq > $e

fcall = (func, keys, vals)=>
  keys.unshift keys.length
  R.fcall(func, keys, vals)

main = ()=>
  host = process.argv[2]
  host = host.trim().toLowerCase()
  if not host
    return

  rhost = reverse(host)
  id = await fcall(
    'zsetId'
    [
      'hostId'
    ]
    [
      rhost
    ]
  )
  await $e(
    "INSERT IGNORE INTO host (id,v,ts) VALUES (?,?,UNIX_TIMESTAMP())"
    id
    host
  )
  console.log host, id
  return

await main()
process.exit()
