wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.headers["Cookie"] = "session_id=YOUR_VALID_SESSION_ID"

request = function()
  return wrk.format(nil, "/auth/logout", nil, nil)
end
