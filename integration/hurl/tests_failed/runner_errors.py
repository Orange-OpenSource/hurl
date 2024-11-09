from io import BytesIO

from app import app
from flask import Response, make_response, redirect


@app.route("/runner_errors")
def runner_errors():
    return "Hello World!"


@app.route("/runner_errors/could_not_uncompress")
def runner_errors_could_not_uncompress():
    result = BytesIO()
    result.write(b"\x01\x02\x03")
    data = result.getvalue()
    resp = make_response(data)
    resp.headers["Content-Encoding"] = "br"
    return resp


@app.route("/runner_errors/invalid-xml")
def runner_errors_invalid_xml():
    return Response("", mimetype="application/xml")


@app.route("/runner_errors/invalid-charset")
def runner_errors_invalid_charset():
    headers = {"Content-Type": "text/html; charset=unknown"}
    return Response("Hello", headers=headers)


@app.route("/runner_errors/invalid-decoding")
def runner_errors_invalid_decoding():
    result = BytesIO()
    result.write(b"\xff")
    data = result.getvalue()
    resp = make_response(data)
    return resp


@app.route("/runner_errors/redirect/2")
def runner_errors_redirect2():
    return redirect("http://localhost:8000/runner_errors/redirect/1")


@app.route("/runner_errors/redirect/1")
def runner_errors_redirect1():
    return redirect("http://localhost:8000/runner_errors")


@app.route("/runner_errors/redirect-custom-scheme")
def runner_errors_redirect_custom_scheme():
    return redirect("market://details?id=com.example.package")


@app.route("/runner_errors/unsupported-content-encoding")
def runner_errors_unsupported_content_encoding():
    headers = {"Content-Encoding": "unknown"}
    return Response("Hello", headers=headers)


@app.route("/runner_errors/json-list")
def runner_errors_json_list():
    return Response("[1,2,3]", mimetype="application/json")
