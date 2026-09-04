from app import app
from flask import request

@app.route("/discard-body")
def discard_body():
    return "Hello World!"
