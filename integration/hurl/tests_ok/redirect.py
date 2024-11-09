from app import app
from flask import Response, redirect


@app.route("/redirect-absolute")
def redirect_absolute():
    return redirect("http://localhost:8000/redirected")


@app.route("/redirect-relative")
def redirect_relative():
    response = Response(status=302)
    response.headers["Location"] = "/redirected"
    response.autocorrect_location_header = False
    return response


@app.route("/redirected")
def redirected():
    return "Redirected"
