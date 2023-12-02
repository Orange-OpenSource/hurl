from app import app
from flask import Response


@app.route("/error-key-template")
def error_key_template():
    return ""
