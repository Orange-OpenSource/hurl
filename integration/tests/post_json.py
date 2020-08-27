from flask import request
from tests import app


@app.route('/post-json', methods=['POST'])
def post_json():
    assert request.headers['Content-Type'] == 'application/json'
    s = request.data.decode("utf-8")
    assert s == '''{
    "name": "Bob",
    "password": "secret"
}'''
    return ''


@app.route('/post-json-array', methods=['POST'])
def post_json_array():
    assert request.headers['Content-Type'] == 'application/json'
    s = request.data.decode("utf-8")
    assert s == '[1,2,3]'
    return ''


@app.route('/post-json-string', methods=['POST'])
def post_json_string():
    assert request.headers['Content-Type'] == 'application/json'
    s = request.data.decode("utf-8")
    assert s == '"Hello"'
    return ''


@app.route('/post-json-number', methods=['POST'])
def post_json_number():
    assert request.headers['Content-Type'] == 'application/json'
    s = request.data.decode("utf-8")
    assert s == '100'
    return ''


@app.route('/post-json-boolean', methods=['POST'])
def post_json_boolean():
    assert request.headers['Content-Type'] == 'application/json'
    s = request.data.decode("utf-8")
    assert s == 'true'
    return ''

@app.route('/post-json-numbers', methods=['POST'])
def post_json_numbers():
    assert request.headers['Content-Type'] == 'application/json'
    s = request.data.decode("utf-8")
    assert s == '''{
    "natural": 100,
    "negative": -1,
    "float": "3.333333333333333",
    "exponent": 100e100
}'''
    return ''

@app.route('/get-name')
def get_name():
    return 'Bob'

