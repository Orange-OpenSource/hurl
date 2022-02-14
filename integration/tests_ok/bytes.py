from app import app
from flask import make_response, request
from io import BytesIO


@app.route("/bytes")
def bytes():
    result = BytesIO()
    result.write(b"\x01\x02\x03")
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "application/octet-stream"
    return resp
