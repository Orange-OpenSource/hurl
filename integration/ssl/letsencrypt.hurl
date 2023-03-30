GET https://hurl.dev
HTTP 200
[Asserts]
certificate "Subject" == "CN=hurl.dev"
certificate "Issuer" == "C=US, O=Let's Encrypt, CN=R3"
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" matches "[0-9af]+"
