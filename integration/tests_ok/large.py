from app import app
from flask import make_response
from io import BytesIO


@app.route("/large")
def large():
    result = BytesIO()
    for _ in range(1024 * 1024 * 32):
        result.write(b"0123456789abcdef")
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "application/octet-stream"
    return resp
