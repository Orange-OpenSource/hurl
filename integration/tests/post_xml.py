# coding=utf-8
from flask import request
from tests import app

@app.route('/post-xml', methods=['POST'])
def post_xml():
    s = request.data.decode("utf-8")
    assert s == '''<?xml version="1.0"?>
<drink>caf\u00e9</drink>'''
    return ''

@app.route('/post-xml-no-prolog', methods=['POST'])
def post_xml_no_prolog():
    s = request.data.decode("utf-8")
    assert s == '''<drink>caf\u00e9</drink>'''
    return ''

