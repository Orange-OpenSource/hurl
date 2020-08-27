from flask import request, make_response
from tests import app
from io import BytesIO


@app.route("/encoding/utf8")
def encoding_utf8():
    return 'café'

@app.route("/encoding/latin1")
def encoding_latin1():
    result = BytesIO()
    result.write(b'\x63\x61\x66\xe9')
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = 'text/html; charset=ISO-8859-1'
    return resp





