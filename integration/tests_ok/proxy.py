from app import app
from flask import request


@app.route("/proxy")
def proxy():
    assert request.headers["Proxy-Connection"] == "Keep-Alive"
    return ""
