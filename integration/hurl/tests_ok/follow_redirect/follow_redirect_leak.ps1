Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --location `
  --header 'Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=' `
  --header 'Cookie: fruit=lemon' `
  --variable trusted=false `
  tests_ok/follow_redirect/follow_redirect_leak.hurl

hurl --location-trusted `
  --header 'Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=' `
  --header 'Cookie: fruit=lemon' `
  --variable trusted=true `
  tests_ok/follow_redirect/follow_redirect_leak.hurl

hurl --location `
  --header 'Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=' `
  --cookie tests_ok/follow_redirect/cookies.txt `
  --variable trusted=false `
  tests_ok/follow_redirect/follow_redirect_leak.hurl

# FIXME: --cookie and --location-trusted should work
#hurl --location-trusted `
#  --header 'Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=' `
#  --cookie tests_ok/follow_redirect/cookies.txt `
#  --variable trusted=true `
#  tests_ok/follow_redirect/follow_redirect_leak.hurl
