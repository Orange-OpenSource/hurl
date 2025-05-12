from app import app
from flask import redirect


@app.route("/redirect-1")
def redirect_1():
    return redirect("http://localhost:8000/redirected")


@app.route("/redirect-2")
def redirect_2():
    return redirect("http://localhost:8000/redirect-1")


@app.route("/redirect-3")
def redirect_3():
    return redirect("http://localhost:8000/redirect-2")
