GET http://localhost:8000/dummy

HTTP 200
[Captures]
count: jsonpath "$.books" count
[Asserts]
jsonpath "$.data" base64Decode == hex,48656c6c6f;                                           # base64Decode
bytes base64Encode == "SGVsbG8="                                                            # base64Encode
jsonpath "$.books" count == 12                                                              # count
certificate "Expire-Date" daysAfterNow > 15                                                 # daysAfterNow
certificate "Start-Date" daysBeforeNow < 100                                                # daysBeforeNow
bytes decode "iso-8859-1" == "café"                                                         # decode
cookie "LSID[Expires]" format "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"        # format
jsonpath "$.text" htmlEscape == "a &gt; b"                                                  # htmlEscape
jsonpath "$.escaped_html[1]" htmlUnescape == "<p>Hello</p>"                                 # htmlUnescape
variable "books" jsonpath "$[0].name" == "Dune"                                             # jsonpath
jsonpath "$.books" nth 2 == "Children of Dune"                                              # nth
body regex /Hello ([0-9]+)!/ == "Bob"                                                       # regex
jsonpath "$.ips" replace ", " "|" == "192.168.2.1|10.0.0.20|10.0.0.10"                      # replace
jsonpath "$.ips" split ", " count == 3                                                      # split
header "Expires" toDate "%a, %d %b %Y %H:%M:%S GMT" daysBeforeNow > 1000                    # toDate
jsonpath "$.pi" toFloat == 3.14                                                             # toFloat
jsonpath "$.id" toInt == 123                                                                # toInt
jsonpath "$.encoded_url" urlDecode == "https://mozilla.org/?x=шеллы"                        # urlDecode
jsonpath "$.url" urlEncode == "https%3A//mozilla.org/%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B" # urlEncode
bytes decode "iso-8859-1" xpath "string(//p)" == "Hello"                                    # xpath
