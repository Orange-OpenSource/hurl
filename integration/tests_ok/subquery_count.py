from app import app
from flask import Response


@app.route("/subquery-count")
def subquery_count():
    return Response(
        """{
  "users": ["Bob", "Bill", "Bruce"]
}""",
        mimetype="application/json",
    )
