from app import app
from flask import request


@app.route("/query-match-none")
def query_match_none():
    return "Hello World!"
