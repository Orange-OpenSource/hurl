GET http://localhost:8000/jsonpath/types
HTTP 200
[Asserts]
jsonpath "$[?@.value == null].name" == "null"
jsonpath "$[?@.value == true].name" == "true"
jsonpath "$[?@.value == false].name" not exists
jsonpath "$[?@.value == 42].name" == "fourty_two"
jsonpath "$[?@.value == -0].name" == "zero"
jsonpath "$[?@.value == 1000000000000].name" == "one_trillion"
jsonpath "$[?@.value == 1e12].name" == "one_trillion"
jsonpath "$[?@.value == 1e+12].name" == "one_trillion"
jsonpath "$[?@.value == 43252003274489856000].name" == "rubikcube_configuration"
jsonpath "$[?@.value == 0.01].name" == "one_hundredth"
jsonpath "$[?@.value == 1e-2].name" == "one_hundredth"
jsonpath "$[?@.value == 'hello'].name" == "hello"
jsonpath "$[?@.value == \"hello\"].name" == "hello"
jsonpath "$[?@.value == \"'\"].name" == "single_quote"
jsonpath "$[?@.value == '\"'].name" == "double_quote"
# jsonpath "$[?@.value == '\t'].name"  == "tab"            TODO
#jsonpath "$[?@.value == '\\'].name"  == "backslash"       TODO
jsonpath "$[?@.value == '🔥'].name" == "fire"
# jsonpath "$[?@.value == '\u0001\uF525'].name"  == "fire" TODO
jsonpath "$[?@.value == '\u{1F525}'].name" == "fire"
jsonpath "$[?@.value == '✈'].name" == "plane"
#jsonpath "$[?@.value == '\u2708'].name"  == "plane"       TODO
jsonpath "$[?@.value == '\u{2708}'].name" == "plane"

# Passing different types to function param is syntactically correct
jsonpath "$[?match(@.name, null)]" not exists
jsonpath "$[?match(@.name, true)]" not exists
jsonpath "$[?match(@.name, false)]" not exists
jsonpath "$[?match(@.name, 42)]" not exists
jsonpath "$[?match(@.name, 'hello')]" exists

[       
  { "name": "null", "value": null },
  { "name": "true", "value": true },
  { "name": "fourty_two", "value": 42 },
  { "name": "zero", "value": 0 },
  { "name": "one_trillion", "value": 1e12 },
  { "name": "rubikcube_configuration", "value": 43252003274489856000},
  { "name": "one_hundredth", "value": 0.01 },
  { "name": "hello", "value": "hello" },
  { "name": "single_quote", "value": "'" },
  { "name": "double_quote", "value": "\"" },
  { "name": "tab", "value": "\t" },
  { "name": "backslash", "value": "\\" },
  { "name": "plane", "value": "✈" },
  { "name": "fire", "value": "🔥" },
  { "name": "array", "value": [1, 2] },
  { "name": "object", "value": { "a": 1 } }
]
