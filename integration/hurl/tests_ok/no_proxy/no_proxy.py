from app import app
from flask import request


@app.route("/no-proxy")
def no_proxy():
    assert "From-Proxy" not in request.headers
    return "Hello World!"
