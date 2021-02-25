from flask import request
from tests import app
import json


@app.route('/variables', methods=['POST'])
def variables():
    assert request.headers['Content-Type'] == 'application/json'
    s = request.data.decode("utf-8")
    data = json.loads(s)
    assert data['name'] == 'Jennifer'
    assert data['age'] == 30
    assert data['female'] == True
    assert data['id'] == '123'
    assert data['height'] == 1.7
    assert data['a_null'] == None
    return ''



