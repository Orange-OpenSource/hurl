from io import BytesIO

from app import app
from flask import make_response


@app.route("/get_large")
def get_large():
    result = BytesIO()
    # Returns ~536M
    for _ in range(1024 * 1024 * 32):
        result.write(b"0123456789abcdef")
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "application/octet-stream"
    return resp
