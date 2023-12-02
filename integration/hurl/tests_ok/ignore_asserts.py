from app import app
from flask import Response


@app.route("/ignore_asserts")
def ignore_asserts():
    return "Hello"
