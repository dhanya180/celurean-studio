wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

math.randomseed(os.time() * os.clock())

request = function()
  local uuid = string.format("%x%x%x", 
    math.random(0, 0xFFFFFFFF),
    math.random(0, 0xFFFFFFFF),
    os.clock() * 1000000
  )
  
  local body = string.format('{"username":"u%s","password":"P","email":"%s@t.com","birth_date":"2000-01-15"}', 
    uuid, uuid)
  
  return wrk.format(nil, nil, nil, body)
end
