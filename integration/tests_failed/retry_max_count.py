from app import app
from flask import request, abort


@app.route("/not-found")
def not_found():
    abort(404)
