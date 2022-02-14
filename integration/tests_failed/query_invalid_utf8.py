from app import app
from flask import make_response
from io import BytesIO


@app.route("/error-query-invalid-utf8")
def error_query_invalid_utf8():
    result = BytesIO()
    result.write(b"\xff")
    data = result.getvalue()
    resp = make_response(data)
    return resp
