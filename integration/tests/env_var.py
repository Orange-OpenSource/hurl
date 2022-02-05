from flask import request
from tests import app


@app.route("/env-var")
def env_var():
    assert request.args.get("name") == "Bob"
    return ""
