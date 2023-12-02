from app import app
from flask import Response


@app.route("/error-filter")
def error_filter():
    return Response(
        """{
          "id":"123x",
          "status": true,
          "list": [1,2,3]
}
""",
        mimetype="application/json",
    )
