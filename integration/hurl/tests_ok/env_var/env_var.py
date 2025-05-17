from app import app
from flask import request


@app.route("/env-var")
def env_var():
    assert request.args.get("name") == "Bob"
    return ""
