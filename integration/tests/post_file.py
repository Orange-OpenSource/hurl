from flask import request
from tests import app

@app.route('/post-file', methods=['POST'])
def post_file():
    assert request.data == b'Hello World!'
    return ''

