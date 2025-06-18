# regex query uses syntax from <https://docs.rs/regex/latest/regex/#syntax>
GET http://localhost:8000/assert-regex
HTTP 200
[Asserts]
regex "Hello ([0-9]+)!" not exists
regex /Hello ([0-9]+)!/ not exists
body regex /Hello ([0-9]+)!/ not exists
regex "Hello ([a-zA-Z]+)!" == "World"
regex /Hello ([a-zA-Z]+)!/ == "World"
# Flags are supported (see <https://docs.rs/regex/latest/regex/#grouping-and-flags>).
# For instance, if you want to capture in a case-insensitive way:
regex /(?i)hello (\w+)!/ == "World"
