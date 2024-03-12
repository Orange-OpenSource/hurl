GET http://localhost:8000/dummy
HTTP 200
[Asserts]
jsonpath "$.book" not == "Dune"
jsonpath "$.book" == "Dune"                  # ==
jsonpath "$.color" != "red"                  # !=
jsonpath "$.year" > 1978                     # >
jsonpath "$.year" >= 1978                    # >=
jsonpath "$.year" < 1978                     # <
jsonpath "$.year" <= 1978                    # <=
jsonpath "$.movie" contains "Empire"         # contains
bytes contains hex,beef;                     # contains
jsonpath "$.movie" endsWith "Back"           # endsWith
bytes endsWith hex,ab123456;                 # endsWith
jsonpath "$.book" exists                     # exists
jsonpath "$.nooks" includes "Dune"           # includes
jsonpath "$.succeeded" isBoolean             # isBoolean
jsonpath "$.books" isCollection              # isCollection
jsonpath "$.publication_date" isDate         # isDate
jsonpath "$.movies" isEmpty                  # isEmpty
jsonpath "$.height" isFloat                  # isFloat
jsonpath "$.count" isInteger                 # isInteger
jsonpath "$.name" isString                   # isString
jsonpath "$.release" matches "\\d{4}"        # matches
jsonpath "$.release" matches /\d{4}/         # matches
jsonpath "$.movie" startsWith "The"          # startsWith
bytes startsWith hex,efbbbf;                 # startsWith
