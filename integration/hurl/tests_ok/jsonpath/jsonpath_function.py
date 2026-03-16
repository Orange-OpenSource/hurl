from app import app
from flask import Response


@app.route("/jsonpath/function")
def jsonpath_function():
    return Response(
        """{
  "two": 2,
  "pattern": "ca.*",
  "string": "ca",
  "items": [
    {
      "id": 1,
      "name": "car",
      "tags": [
        "transport"
      ],
      "heavy": true
    },
    {
      "id": 2,
      "name": "bike",
      "tags": [
        "transport",
        "sport"
      ],
      "heavy": false
    },
    {
      "id": 3,
      "name": "plane",
      "tags": [
        "transport",
        "fast",
        "expensive"
      ],
      "heavy": true
    }
  ]
}""",
        mimetype="application/json",
    )
