from io import BytesIO

from app import app
from flask import make_response


@app.route("/rawbytes")
def rawbytes():
    result = BytesIO()
    result.write(b"\x01\x02\x03")
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "application/octet-stream"
    return resp


@app.route("/rawbytes/gzip")
def rawbytes_gzip():
    result = BytesIO()
    result.write(
        b"\x1f\x8b\x08\x00\xed\x0c\x84\x5f\x00\x03\xf3\x48\xcd\xc9\xc9\x57\x08\xcf\x2f\xca\x49\x51\x04\x00\xa3\x1c\x29\x1c\x0c\x00\x00\x00"
    )
    data = result.getvalue()
    resp = make_response(data)
    resp.headers["Content-Encoding"] = "gzip"
    return resp


@app.route("/rawbytes/empty")
def rawbytes_empty():
    result = BytesIO()
    result.write(b"")
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "application/octet-stream"
    return resp
