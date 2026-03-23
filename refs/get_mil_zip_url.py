#!/usr/bin/python3
"""Get Military/Security resource URL from Open Government Portal CKAN API."""
import ssl, urllib.request, json

ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE

url = 'https://open.canada.ca/api/action/resource_show?id=99a220a8-fa42-4231-9aa9-c626135e0912'
req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
try:
    data = json.loads(urllib.request.urlopen(req, context=ctx, timeout=15).read())
    result = data.get('result', {})
    print('Name:', result.get('name'))
    print('URL:', result.get('url'))
    print('Format:', result.get('format'))
    print('Size:', result.get('size'))
except Exception as e:
    print('Error:', e)

# Also try dataset endpoint
url2 = 'https://open.canada.ca/api/action/package_show?id=94fc74d6-9b9a-4c2e-9c6c-45a5092453aa'
req2 = urllib.request.Request(url2, headers={'User-Agent': 'Mozilla/5.0'})
try:
    data2 = json.loads(urllib.request.urlopen(req2, context=ctx, timeout=15).read())
    resources = data2.get('result', {}).get('resources', [])
    for r in resources:
        print(f"{r.get('name', '?')[:50]:50s} -> {r.get('url', '?')}")
except Exception as e:
    print('Dataset error:', e)
