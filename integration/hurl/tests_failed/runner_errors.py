from app import app
from flask import make_response, Response
from io import BytesIO


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


@app.route("/runner_errors/json-list")
def runner_errors_json_list():
    return Response("[1,2,3]", mimetype="application/json")
