GET https://hurl.dev
HTTP 200
[Asserts]
certificate "Subject" replace " = " "=" replace ";" ", " == "CN=hurl.dev"
certificate "Issuer"  replace " = " "=" replace ";" ", " matches "^C=US, O=Let's Encrypt, CN=R1[0-1]$"
certificate "Expire-Date" isDate
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" matches /^([\da-f]{2}:){17}[\da-f]{2}$/
