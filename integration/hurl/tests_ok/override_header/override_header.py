from app import app
from flask import request


@app.route("/override-header")
def override_user_agent():
    assert request.headers.get("User-Agent") == "different-user-agent"
    assert request.headers.get("Accept") == "different-accept"
    assert request.headers.get("Host") == "different-host"
    return ""
