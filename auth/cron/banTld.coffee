#!/usr/bin/env coffee

> @3-/req/reqJson.js
  @3-/reverse
  @3-/msgpack/pack.js
  @3-/msgpack/unpack.js
  @3-/write
  @3-/redis/R.js
  ./conf > PWD
  path > join
  tld-extract:tld
  punycode

PROXY='https://mirror.ghproxy.com/'
DAY = 86400
WEEK = DAY * 7
KEY = 'banTld'
UPDATE = KEY+':update'
FAKEFILTER = KEY+':fakefilter'

< main = =>
  update = await R.get UPDATE
  now = Math.round new Date()/1000
  if update
    update = parseInt update,36
    diff = WEEK - (now - update)
    if diff > 0
      console.log KEY+' next update after '+Math.round(1000*diff/DAY)/1000+' days'
      return

  url = PROXY+'https://raw.githubusercontent.com/7c/fakefilter/main/json/data.json'
  {t,domains} = await reqJson url

  li = new Set
  for [host,o] from Object.entries domains
    {lastseen} = o
    if (t - lastseen)/86400 < 365
      host = tld 'http://'+host, allowUnknownTLD:true
      host = punycode.toUnicode(host.domain.trim().toLowerCase())
      if host.length
        host = reverse host
        li.add host

  li = [...li]
  li.sort()

  p = R.pipeline()

  toadd = []
  fakefilter = await R.getBuffer(FAKEFILTER)
  if fakefilter # 删除过期的，不直接删除KEY，是防止有将来可以手工添加屏蔽域名
    fakefilter = new Set unpack(fakefilter)
    for i from li
      if not fakefilter.delete i
        toadd.push i

    {size} = fakefilter
    if fakefilter.size
      p.srem KEY,...fakefilter
      console.log 'remove', size
  else
    toadd = li

  {length} = toadd

  if length
    p.sadd KEY, ...toadd
    console.log 'add', length

  p.set FAKEFILTER, pack(li)
  p.set UPDATE,now.toString 36
  p.exec()
  return

if process.argv[1] == decodeURI(new URL(import.meta.url).pathname)
  await main()
  process.exit()
