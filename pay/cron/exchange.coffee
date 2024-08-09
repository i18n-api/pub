#!/usr/bin/env coffee

> @3-/dbq > $e $one $q
  @3-/req/reqJson.js
  @3-/utf8/utf8d.js
  @3-/redis/R.js
  @3-/retry
  # @3-/exchange

BASE = 1e6
FROM_CURRENCY = 'USD'
TO_CURRENCY = 'EUR'

rate = (amount, fee, date, from_curr, to_curr) =>
  encoded_date = encodeURIComponent date
  url = "https://www.visa.com.hk/cmsapi/fx/rates?amount=#{amount}&fee=#{fee}&utcConvertedDate=#{encoded_date}&exchangedate=#{encoded_date}&fromCurr=#{from_curr}&toCurr=#{to_curr}"
  console.log url
  reqJson(
    url
    headers:
      'accept': 'application/json, text/plain, */*'
      'referer': 'https://www.visa.com.hk/en_HK/support/consumer/travel-support/exchange-rate-calculator.html'
      'user-agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36'
  )

mdy = (time)=>
  d = new Date(time)
  month = String(d.getMonth() + 1).padStart(2, '0')
  day = String(d.getDate()).padStart(2, '0')
  year = d.getFullYear()
  return [month,day,year].join '/'


run = retry =>
  today = new Date() - 288e5
  r = await rate 1, 2, mdy(today), FROM_CURRENCY, TO_CURRENCY
  rate = Number.parseFloat r.reverseAmount
  if rate < 0.5 or rate > 2
    throw new Error "#{FROM_CURRENCY} -> #{TO_CURRENCY} rate error #{rate}"
  rate = BASE*rate
  day = Math.floor today / 864e5
  ID = new Map (await $q("SELECT v,id FROM payCurrency")).map ([v,id])=>
    [
      utf8d v
      id
    ]
#   [day,rate]= await exchange(FROM_CURRENCY,TO_CURRENCY)

  await $e(
    "INSERT INTO payExchangeRate(f,t,day,v)VALUES(?,?,?,?) ON DUPLICATE KEY UPDATE id=id"
    ID.get FROM_CURRENCY
    ID.get TO_CURRENCY
    day
    rate
  )

  console.log FROM_CURRENCY, '->',TO_CURRENCY,day,rate
  await R.hset "conf",FROM_CURRENCY+TO_CURRENCY,rate
  return

await run()

process.exit()
