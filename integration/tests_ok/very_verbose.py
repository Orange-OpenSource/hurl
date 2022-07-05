from io import BytesIO

from app import app
from flask import Response, make_response, request


def redirect(location, status=301):
    body = f"""<html>
<head>
    <meta content="text/html;charset=utf-8">
    <title>301 Moved</title>
</head>
<body>
<h1>{status} Moved</h1>
The document has moved
<a href="{location}">here</a>.
</body>
</html>"""
    response = Response(response=body, status=status, mimetype="text/html")
    response.headers["Location"] = location
    return response


@app.route("/very-verbose/redirect")
def very_verbose_redirect():
    return redirect(location="/very-verbose/redirected")


@app.route("/very-verbose/redirected")
def very_verbose_redirected():
    return "Redirected."


@app.route("/very-verbose/encoding/latin1")
def very_verbose_encoding_latin1():
    result = BytesIO()
    result.write(b"\x63\x61\x66\xe9")  # café in ISO-8859-1
    data = result.getvalue()
    resp = make_response(data)
    resp.content_type = "text/html; charset=ISO-8859-1"
    return resp


@app.route("/very-verbose/compressed/brotli", methods=["GET", "POST"])
def very_verbose_compressed_brotli():
    assert "br" in request.headers["Accept-Encoding"]
    result = BytesIO()
    result.write(
        b"\x21\x2c\x00\x04\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21\x03"
    )
    data = result.getvalue()
    resp = make_response(data)
    resp.headers["Content-Encoding"] = "br"
    return resp


@app.route("/very-verbose/cat")
def very_verbose_cat():
    with open("tests_ok/cat.jpg.br", "rb") as f:
        data = f.read()
    resp = make_response(data)
    resp.headers["Content-Type"] = "image/jpeg"
    return resp


@app.route("/very-verbose/update-cat", methods=["POST"])
def very_verbose_update_cat():
    upload = request.files["cat"]
    assert upload.filename == "cat.jpg"
    assert upload.content_type == "image/jpeg"
    return ""


@app.route("/very-verbose/done")
def very_verbose_done():
    return "Done"
