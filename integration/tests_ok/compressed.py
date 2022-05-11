from app import app
from flask import make_response, request
from io import BytesIO


@app.route("/compressed/gzip")
def compressed_gzip():
    assert "gzip" in request.headers["Accept-Encoding"]
    result = BytesIO()
    result.write(
        b"\x1f\x8b\x08\x00\xed\x0c\x84\x5f\x00\x03\xf3\x48\xcd\xc9\xc9\x57\x08\xcf\x2f\xca\x49\x51\x04\x00\xa3\x1c\x29\x1c\x0c\x00\x00\x00"
    )
    data = result.getvalue()
    resp = make_response(data)
    resp.headers["Content-Encoding"] = "gzip"
    return resp


@app.route("/compressed/zlib")
def compressed_zlib():
    assert "deflate" in request.headers["Accept-Encoding"]
    result = BytesIO()
    result.write(
        b"\x78\x9c\xf3\x48\xcd\xc9\xc9\x57\x08\xcf\x2f\xca\x49\x51\x04\x00\x1c\x49\x04\x3e"
    )
    data = result.getvalue()
    resp = make_response(data)
    resp.headers["Content-Encoding"] = "deflate"
    return resp


@app.route("/compressed/brotli")
def compressed_brotli():
    assert "br" in request.headers["Accept-Encoding"]
    result = BytesIO()
    result.write(
        b"\x21\x2c\x00\x04\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21\x03"
    )
    data = result.getvalue()
    resp = make_response(data)
    resp.headers["Content-Encoding"] = "br"
    return resp


@app.route("/compressed/brotli_large")
def compressed_brotli_large():
    assert "br" in request.headers["Accept-Encoding"]
    with open("tests_ok/cat.jpg.br", "rb") as f:
        data = f.read()
    resp = make_response(data)
    resp.headers["Content-Encoding"] = "br"
    resp.headers["Content-Type"] = "image/jpeg"
    return resp


@app.route("/compressed/brotli_identity")
def compressed_brotli_identity():
    assert "br" in request.headers["Accept-Encoding"]
    result = BytesIO()

    result.write(
        b"\x21\x2c\x00\x04\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21\x03"
    )
    data = result.getvalue()
    resp = make_response(data)
    resp.headers["Content-Encoding"] = "br, identity"
    return resp


@app.route("/compressed/none")
def compressed_none():
    return "Hello World!"
