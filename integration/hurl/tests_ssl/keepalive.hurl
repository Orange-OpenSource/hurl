# On reused connection, libcurl doesn't provide SSL certificate information, we can only get this from
# the first connection.
# Because of this, Hurl has to keep cache of certificate information by connection. This allows to make certificates
# asserts on reused connection even if libcurl doesn't provide these information.
# Note: this is only possible from curl 8.2.0
#
# See:
#
# - <https://curl.se/libcurl/c/CURLINFO_CONN_ID.html>
# - <https://github.com/Orange-OpenSource/hurl/issues/3031>
# - <https://curl.se/mail/lib-2024-07/0007.html>
#

GET https://hurl.dev
HTTP 200
[Asserts]
header "Connection" not exists
certificate "Subject" replace " = " "=" replace ";" ", " == "CN=hurl.dev"
certificate "Issuer"  replace " = " "=" replace ";" ", " matches "^C=US, O=Let's Encrypt, CN=R1[0-1]$"
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" matches /^([\da-f]{2}:){17}[\da-f]{2}$/


GET https://hurl.dev
HTTP 200
[Asserts]
certificate "Subject" replace " = " "=" replace ";" ", " == "CN=hurl.dev"
certificate "Issuer"  replace " = " "=" replace ";" ", " matches "^C=US, O=Let's Encrypt, CN=R1[0-1]$"
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" matches /^([\da-f]{2}:){17}[\da-f]{2}$/


GET https://gitlab.com
HTTP *
[Asserts]
header "Connection" not exists
certificate "Subject" contains "gitlab.com"
certificate "Expire-Date" daysAfterNow > 15


GET https://gitlab.com
HTTP *
[Asserts]
header "Connection" not exists
certificate "Subject" contains "gitlab.com"
certificate "Expire-Date" daysAfterNow > 15


GET https://google.com
HTTP *
[Asserts]
header "Connection" not exists
certificate "Subject" contains "google.com"
certificate "Expire-Date" daysAfterNow > 15


GET https://google.com
HTTP *
[Asserts]
header "Connection" not exists
certificate "Subject" contains "google.com"
certificate "Expire-Date" daysAfterNow > 15

