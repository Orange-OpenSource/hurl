from app import app
from flask import request


@app.route("/proxy")
def proxy():
    print(request.headers)
    assert request.headers["From-Proxy"] == "Hello"
    return ""
