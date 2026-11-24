Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --location `
  --header 'Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=' `
  --header 'Cookie: fruit=lemon' `
  --variable expect_auth_forwarded=false `
  --variable expect_cookie_forwarded=false `
  tests_ok/follow_redirect/follow_redirect_leak.hurl

hurl --location-trusted `
  --header 'Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=' `
  --header 'Cookie: fruit=lemon' `
  --variable expect_auth_forwarded=true `
  --variable expect_cookie_forwarded=true `
  tests_ok/follow_redirect/follow_redirect_leak.hurl

hurl --location `
  --header 'Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=' `
  --cookie tests_ok/follow_redirect/cookies.txt `
  --variable expect_auth_forwarded=false `
  --variable expect_cookie_forwarded=false `
  tests_ok/follow_redirect/follow_redirect_leak.hurl

# Cookies from a cookie file are domain-scoped: they are not forwarded to another host,
# even with --location-trusted (same as curl).
hurl --location-trusted `
  --header 'Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=' `
  --cookie tests_ok/follow_redirect/cookies.txt `
  --variable expect_auth_forwarded=true `
  --variable expect_cookie_forwarded=false `
  tests_ok/follow_redirect/follow_redirect_leak.hurl
