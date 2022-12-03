from app import app
from flask import Response


@app.route("/error-filter")
def error_filter():
    return Response(
        """{
          "id":"123x",
          "status": true
}
""",
        mimetype="application/json",
    )
