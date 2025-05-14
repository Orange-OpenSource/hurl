import re

from app import app
from flask import request


@app.route("/aws-sigv4", methods=["POST"])
def aws_sigv4():
    auth = request.headers.get("Authorization")
    assert re.match(
        r"^AWS4-HMAC-SHA256 Credential=someAccessKeyId/\d+/eu-central-1/hurltest/aws4_request, SignedHeaders=\S+, Signature=[a-f0-9]+$",
        auth,
    )
    return ""
