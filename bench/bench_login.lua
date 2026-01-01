wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

request = function()
  local body = '{"email":"s@t.com","password":"P"}'
  return wrk.format(nil, nil, nil, body)
end
