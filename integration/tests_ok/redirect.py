from app import app
from flask import redirect


@app.route("/redirect")
def redirectme():
    return redirect("http://localhost:8000/redirected")


@app.route("/redirected")
def redirected():
    return ""
