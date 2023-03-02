wrk.method = "POST"
-- wrk.body  =  "{\"operationName\": null, \"variables\": {}, \"query\": \"mutation {\n  register(username: \"salva\", password: \"hell0021\") {\n    ok\n  }\n}\n}"
-- wrk.body = "{\"operationName\":null,\"variables\":{},\"query\":\"mutation {\n  register(username: \"salva\", password: \"hell0021\") {\n    ok\n  }\n}\n\"}"
query = [[
    mutation register($username: String!, $password: String!){
        register(username:$username, password:$password){
          ok
        }
      }
]]
variables = [[
    {
    "username": "salva",
    "password": "hell0021"
    }
]]
wrk.body   ='{"query": "' .. string.gsub(query, '\n', '') .. '", "variables": ' .. string.gsub(variables, '\n', '') .. ' }'
wrk.headers["Content-Type"] = "json/application"
