GET https://google.com
GET https://github.com
GET https://gitlab.com

# We test that certificates in cache are set by connection. Need curl >= 8.2.0
# See <https://github.com/Orange-OpenSource/hurl/issues/5414>
HEAD https://hurl.dev
[Options]
skip: {{skip_cert_test}}
HTTP 200
[Asserts]
certificate "Subject" contains "hurl.dev"

HEAD https://gitlab.com
[Options]
skip: {{skip_cert_test}}
HTTP *
[Asserts]
certificate "Subject" contains "gitlab.com"

HEAD https://hurl.dev
[Options]
skip: {{skip_cert_test}}
HTTP 200
[Asserts]
certificate "Subject" contains "hurl.dev"
