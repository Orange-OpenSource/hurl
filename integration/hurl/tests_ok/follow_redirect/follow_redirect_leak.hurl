GET http://localhost:8000/follow-redirect-leak/host-a-step-1
HTTP 200
[Asserts]
redirects count == 2
redirects nth 0 location == "http://127.0.0.1:8000/follow-redirect-leak/host-b-step-2"
redirects nth 1 location == "http://localhost:8000/follow-redirect-leak/host-a-step-3"
url == "http://localhost:8000/follow-redirect-leak/host-a-step-3"
header "Location" not exists
`Followed redirect!`


