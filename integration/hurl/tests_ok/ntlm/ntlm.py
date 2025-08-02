import base64
import struct

from app import app
from flask import request

username = "username"
domain = "domain"
password = "password"


@app.route("/ntlm")
def ntlm_auth():
    return get_auth_response("NTLM")


def get_auth_response(auth_type, advertise_nego_and_ntlm=False, no_challenge=False):
    # Get the actual header that is returned by requests_ntlm
    actual_header = request.headers.get("Authorization", "")

    # Check what the message type is from the header
    if actual_header == "":
        # This is the initial connection, need to return a 401
        response_headers = {
            "WWW-Authenticate": (
                auth_type if not advertise_nego_and_ntlm else "Negotiate, NTLM"
            )
        }
        status_code = 401
        response = "auth with '%s\\%s':'%s'" % (domain, username, password)
    else:
        # Set human readable names for message types
        # see https://msdn.microsoft.com/en-us/library/cc236639.aspx for more details
        expected_signature = b"NTLMSSP\x00"
        negotiate_message_type = 1
        authenticate_message_type = 3

        msg = base64.b64decode(actual_header[len(auth_type) :])
        signature = msg[0:8]
        if signature != expected_signature:
            raise ValueError(
                "Mismatch on NTLM message signature, expecting: %s, actual: %s"
                % (expected_signature, signature)
            )
        # Get the NTLM version number (bytes 9 - 12)
        message_type = struct.unpack("<I", msg[8:12])[0]

        if no_challenge:
            response_headers = {"WWW-Authenticate": auth_type}
            response = "access denied"
            status_code = 401
        elif message_type == negotiate_message_type:
            # Initial NTLM message from client, attach challenge token
            challenge_response = (
                "TlRMTVNTUAACAAAAAwAMADgAAAAzgoriASNFZ4mrze8AAAA"
                "AAAAAACQAJABEAAAABgBwFwAAAA9TAGUAcgB2AGUAcgACAA"
                "wARABvAG0AYQBpAG4AAQAMAFMAZQByAHYAZQByAAAAAAA="
            )
            challenge_header = auth_type + " " + challenge_response
            response_headers = {"WWW-Authenticate": challenge_header}
            response = "auth with '%s\\%s':'%s'" % (domain, username, password)
            status_code = 401
        elif message_type == authenticate_message_type:
            # Received final NTLM message, return 200
            response_headers = {}
            status_code = 200
            response = "authed"
        else:
            # Should only ever receive a negotiate (1) or auth (3) message from requests_ntlm
            raise ValueError(
                "Mismatch on NTLM message type, expecting: 1 or 3, actual: %d"
                % message_type
            )

    return response, status_code, response_headers
