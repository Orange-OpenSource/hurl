from tests import app
from flask import request

@app.route("/querystring-params")
def querystring_params():
    assert request.args.get('param1') == 'value1'
    assert request.args.get('param2') == ''
    assert request.args.get('param3') == 'a=b'
    assert request.args.get('param4') == '1,2,3'
    return ''

@app.route("/querystring-params-encoded")
def querystring_params_encoded():
    assert request.args.get('value1') == '/'
    assert request.args.get('value2') == '/'
    assert request.args.get('value3') == '/'
    return ''
