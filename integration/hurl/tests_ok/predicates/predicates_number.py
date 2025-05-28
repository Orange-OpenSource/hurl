from app import app
from flask import Response


@app.route("/predicates-number")
def predicates_number():
    return Response(
        """{
  "integer": 1,
  "float": 1.0,
  "small_float1": 0.1,
  "small_float2": 0.100000000000000005,
  "big_float1": 1000000000000000000000.0,
  "big_float2": 1000000000000000000000.5,
  "big_integer": 1000000000000000000000
}""",
        mimetype="application/json",
    )
