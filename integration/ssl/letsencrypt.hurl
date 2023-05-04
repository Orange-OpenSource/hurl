GET https://hurl.dev
HTTP 200
[Asserts]
certificate "Subject" replace " = " "=" replace ";" ", " == "CN=hurl.dev"
certificate "Issuer"  replace " = " "=" replace ";" ", " == "C=US, O=Let's Encrypt, CN=R3"
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" matches /^([\da-f]{2}:){17}[\da-f]{2}$/
