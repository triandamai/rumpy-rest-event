```toml
name = 'verify-otp'
description = '{{base_url}}/auth/verify-otp'
method = 'POST'
url = '{{base_url}}/auth/verify-otp'
sortWeight = 2000000
id = '96aa9c3a-2a8d-43bf-aab4-104ba68cc8a8'

[auth.bearer]
token = '{{session_token}}'

[body]
type = 'JSON'
raw = '''
{
    "otp":"691580"
}'''
```

#### Post-response Script

```js
let body = JSON.parse(jc.response.text())
if(jc.response.code >= 200 && jc.response.code <= 209){
    jc.environment.set("session_token",body.data.token)
    jc.variables.set("session_token",body.data.token)
}

jc.test(`${jc.response.code} tes`,()=>{

})
```
