#!/usr/bin/env coffee

# > @3-/redis/R.js
#
# key = 'uid'
# uid = await R.incr key
#
# min = 128
# if uid < min
#   await R.set key, min
#   console.log 'init '+key, min
#
# process.exit()
