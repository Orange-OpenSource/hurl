# coding=utf-8
from flask import request,make_response
from tests import app

@app.route("/default-headers")
def default_headers():
    print('> host:' + request.headers['Host'] + "'")
    assert 'hurl' in request.headers['User-Agent']
    assert request.headers['Host'] == 'localhost:8000'
    assert 'Content-Length' not in request.headers
    return ''


@app.route("/custom-headers")
def custom_headers():
    print(request.headers)
    # TODO: what is expected when request header has multiple values ?
    assert request.headers['Fruit'] == "Raspberry,Apple,Banana,Grape"
    assert request.headers['Color'] == 'Green'
    return ''


@app.route("/custom-headers-utf8")
def custom_headers_utf8():
    print(request.headers)
    assert len(request.headers['Beverage']) == 5
    assert request.headers['Beverage'] == '\x63\x61\x66\xc3\xa9'
    return ''


@app.route("/response-headers")
def response_headers():
    resp = make_response()
    #resp.headers['Beverage'] = '\x63\x61\x66\xc3\xa9'
    resp.headers['Beverage'] = 'cafe'
    return resp