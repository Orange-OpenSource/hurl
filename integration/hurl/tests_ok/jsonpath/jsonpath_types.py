from app import app
from flask import Response


@app.route("/jsonpath/types")
def jsonpath_types():
    return Response(
        """[       
  { "name": "null", "value": null },
  { "name": "true", "value": true },
  { "name": "fourty_two", "value": 42 },
  { "name": "zero", "value": 0 },
  { "name": "one_trillion", "value": 1e12 },
  { "name": "rubikcube_configuration", "value": 43252003274489856000},
  { "name": "one_hundredth", "value": 0.01 },
  { "name": "hello", "value": "hello" },
  { "name": "single_quote", "value": "'" },
  { "name": "double_quote", "value": "\\"" },
  { "name": "tab", "value": "\\t" },
  { "name": "backslash", "value": "\\\\" },
  { "name": "plane", "value": "✈" },
  { "name": "fire", "value": "🔥" },
  { "name": "array", "value": [1, 2] },
  { "name": "object", "value": { "a": 1 } }
]""",
        mimetype="application/json",
    )
