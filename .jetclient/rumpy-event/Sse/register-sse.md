```toml
name = 'register-sse'
method = 'GET'
url = '{{base_url}}/sse/register?user_id=2121&device_id=3243'
sortWeight = 1000000
id = 'af34605b-590b-49e4-acb0-7a784c719d48'

[[queryParams]]
key = 'user_id'
value = '2121'

[[queryParams]]
key = 'device_id'
value = '3243'

[auth.bearer]
token = '{{session_token'
```
