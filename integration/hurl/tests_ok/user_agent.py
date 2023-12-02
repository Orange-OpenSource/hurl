from app import app
from flask import request


@app.route("/user-agent/a")
def useragent_a():
    assert "Mozilla/5.0 A" == request.headers["User-Agent"]
    return ""


@app.route("/user-agent/b")
def useragent_b():
    assert "Mozilla/5.0 B" == request.headers["User-Agent"]
    return ""
