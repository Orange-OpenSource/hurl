from app import app
from flask import Response


@app.route("/error-filter-in-capture")
def error_filter_in_capture():
    return Response(
        """{
          "id":"123x",
          "status": true,
          "list": [1,2,3]
}
""",
        mimetype="application/json",
    )
