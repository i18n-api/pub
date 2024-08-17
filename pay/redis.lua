function payN(KEYS, ARGS)
	local payN = unpack(KEYS)
	local uid, n, threshold = unpack(ARGS)
	local n = tonumber(n)
	local threshold = tonumber(threshold)
	local total = HINCRBY(payN, uid, n)

	if total >= threshold then
		HSET(payN, uid, total % threshold)
		return math.floor(total / threshold)
	end
end
