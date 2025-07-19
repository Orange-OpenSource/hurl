from app import app
from flask import Response


@app.route("/error-filter")
def error_filter():
    return Response(
        """{
          "file":"5L2g5aW95LiW5",
          "base64_string":"SGVsbG8gV29ybGQ=",
          "id":"123x",
          "status": true,
          "list": [1,2,3],
          "empty_list": [],
          "number": 42,
          "invalid_xml": "<?xml version=\\"1.0\\"",
          "date": "2023-01-23T18:25:43.511Z",
          "big_int": 10000000000000000365
}
""",
        mimetype="application/json",
    )
