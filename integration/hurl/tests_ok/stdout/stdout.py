# coding=utf-8
from app import app


@app.route("/stdout/text")
def stdout_text():
    return "Hello"
