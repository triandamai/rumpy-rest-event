```toml
name = 'create-event'
method = 'POST'
url = '{{base_url}}/event/create'
sortWeight = 2000000
id = '04e7cd2f-6a54-4d56-98f6-5cb4e2483640'

[auth.bearer]
token = '{{session_token}}'

[body]
type = 'JSON'
raw = '''
{
  "title": "My first event"
}'''
```
