from app import app
from flask import Response, make_response


@app.route("/large/html")
def large_html():
    data = open("tests_ok/query_cache.html.gz", "rb")
    resp = make_response(data)
    resp.headers["Content-Encoding"] = "gzip"
    return resp
