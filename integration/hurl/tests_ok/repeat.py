from app import app
from flask import request


@app.route("/repeat/hello")
def repeat_hello():
    name = request.args.get("name")
    return f"Hello {name}!\n"
