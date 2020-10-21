from tests import app
from flask import request

@app.route('/basic-authentication')
def basic_authentication():
    assert request.headers['Authorization'] == 'Basic Ym9iOnNlY3JldA=='
    return 'You are authenticated'



