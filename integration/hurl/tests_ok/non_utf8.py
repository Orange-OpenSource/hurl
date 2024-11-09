from io import BytesIO

from app import app
from flask import make_response


@app.route("/non-utf8")
def non_utf8():
    result = BytesIO()
    result.write(b"\x41\x0a\xe9\x0a\xaa")
    data = result.getvalue()
    resp = make_response(data)
    return resp
