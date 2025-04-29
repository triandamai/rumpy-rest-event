```toml
name = 'auth-otp'
description = '{{base_url}}/auth/otp'
method = 'POST'
url = '{{base_url}}/auth/otp'
sortWeight = 1000000
id = '680eed1d-ba8c-4a01-aff4-a655bcd4972c'

[body]
type = 'JSON'
raw = '''
{
    "phone_number":"081226809435"
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
