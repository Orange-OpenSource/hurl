GET https://hurl.dev
HTTP 200
[Asserts]
certificate "Subject" == "CN=hurl.dev"
certificate "Issuer" == "C=US, O=Let's Encrypt, CN=R3"
certificate "Expire-Date" daysAfterNow > 15
certificate "Serial-Number" == "03:db:c4:f3:c8:5e:13:66:cc:52:a3:69:2c:fa:04:a0:7b:b1"
