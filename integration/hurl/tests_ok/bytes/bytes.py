from io import BytesIO

from app import app
from flask import make_response


@app.route("/bytes")
def bytes():
    result = BytesIO()
    result.write(b"\x01\x02\x03")
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "application/octet-stream"
    return resp


@app.route("/empty_bytes")
def empty_bytes():
    result = BytesIO()
    result.write(b"")
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "application/octet-stream"
    return resp
