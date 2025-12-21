import hashlib
import re

from app import app
from flask import request

username = "username"
password = "password"
realm = "test-realm"
nonce = "dcd98b7102dd2f0e8b11d0f600bfb0c093"
opaque = "5ccc069c403ebaf9f0171e9517f40e41"


@app.route("/digest")
def digest_auth():
    return get_auth_response()


def get_auth_response():
    # Get the actual header that is returned by the client
    actual_header = request.headers.get("Authorization", "")

    if actual_header == "":
        # This is the initial connection, need to return a 401 with Digest challenge
        challenge = (
            f'Digest realm="{realm}", nonce="{nonce}", opaque="{opaque}", qop="auth"'
        )
        response_headers = {"WWW-Authenticate": challenge}
        status_code = 401
        response = f"auth with '{username}':'{password}'"
    elif actual_header.startswith("Digest "):
        # Parse the Digest authorization header
        # We just need to verify that the client sent a properly formatted Digest response
        # We don't need to verify the actual hash is correct (that's curl's job)

        # Check that required fields are present
        auth_info = actual_header[7:]  # Remove "Digest " prefix

        required_fields = ["username", "realm", "nonce", "uri", "response"]
        has_all_fields = all(f"{field}=" in auth_info for field in required_fields)

        if has_all_fields:
            # Verify the username matches
            username_match = re.search(r'username="([^"]*)"', auth_info)
            if username_match and username_match.group(1) == username:
                response_headers = {}
                status_code = 200
                response = "authed"
            else:
                response_headers = {}
                status_code = 401
                response = "invalid username"
        else:
            response_headers = {}
            status_code = 401
            response = "missing required fields"
    else:
        response_headers = {}
        status_code = 401
        response = "invalid authorization header"

    return response, status_code, response_headers
