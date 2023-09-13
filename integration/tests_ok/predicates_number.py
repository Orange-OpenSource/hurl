from app import app
from flask import Response


@app.route("/predicates-number")
def predicates_number():
    return Response(
        """{
  "integer": 1,
  "float": 1.0
}""",
        mimetype="application/json",
    )
