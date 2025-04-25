GET http://localhost:8000/redirect-3
[Options]
location: true
HTTP 200
[Asserts]
redirects count == 3
redirects nth 0 location == "http://localhost:8000/redirect-2"
redirects nth 1 location == "http://localhost:8000/redirect-1"
redirects nth 2 location == "http://localhost:8000/redirected"
redirects nth 0 location contains "redirect-2"
redirects nth 1 location endsWith "1"
redirects nth 2 location matches /^http:\/\/.*redirected$/
url == "http://localhost:8000/redirected"
`Redirected`
