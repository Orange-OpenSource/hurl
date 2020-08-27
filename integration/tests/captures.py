from tests import app
from flask import make_response, request


@app.route('/captures')
def captures():
    resp = make_response()
    resp.headers['Header1'] = 'value1'
    resp.headers['Header2'] = 'Hello Bob!'
    return resp


@app.route('/captures-check')
def captures_check():
    assert request.args.get('param1') == 'value1'
    assert request.args.get('param2') == 'Bob'
    return ''



@app.route('/captures-json')
def captures_json():
    return '{ "a_null": null, "an_object": {"id": "123"}, "a_list": [1,2,3], "an_integer": 1, "a_float": 1.1, "a_bool": true, "a_string": "hello" }'