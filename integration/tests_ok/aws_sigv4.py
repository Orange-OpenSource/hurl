from flask import request
from app import app

@app.route("/aws-sigv4", methods=["POST"])
def aws_sigv4():
    auth = request.headers.get('Authorization')
    return f"Received Authorization header: {auth}"

