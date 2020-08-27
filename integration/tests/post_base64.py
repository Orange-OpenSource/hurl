from flask import request
from tests import app

@app.route('/post-base64', methods=['POST'])
def post_base64():
    assert request.data == b'Hello World!'
    return ''

