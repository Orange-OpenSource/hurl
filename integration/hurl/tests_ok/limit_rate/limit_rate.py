from io import BytesIO

from app import app
from flask import make_response


@app.route("/dummy_bytes")
def get_dummy_bytes():
    result = BytesIO()
    for _ in range(30_000_000):
        result.write(b"a")
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "application/octet-stream"
    return resp
