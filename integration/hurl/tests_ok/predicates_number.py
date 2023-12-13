from app import app
from flask import Response


@app.route("/predicates-number")
def predicates_number():
    return Response(
        """{
  "integer": 1,
  "float": 1.0,
  "float2": 0.1,
  "float3": 0.100000000000000005
}""",
        mimetype="application/json",
    )
