```toml
name = 'resend-otp'
description = '{{base_url}}/auth/resend-otp'
method = 'POST'
url = '{{base_url}}/auth/resend-otp'
sortWeight = 3000000
id = '2c026da8-d316-44d6-a970-4913afdda99e'

[auth.bearer]
token = '{{session_token}}'

[body]
type = 'JSON'
raw = '''
{
    "otp":"505693"
}'''
```
