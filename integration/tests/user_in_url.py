from tests import app
from flask import request

@app.route('/user-in-url')
def user_in_url():
    assert request.headers['Authorization'] == 'Basic Ym9iOnNlY3JldA=='
    return 'You are authenticated'



