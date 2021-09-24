from flask import request
from tests import app

@app.route('/predicates-string')
def predicates_string():
    return 'Hello World!'

@app.route('/predicates-string-empty')
def predicates_string_empty():
    return ''

@app.route('/predicates-string-unicode')
def predicates_string_unicode():
    return '\u2708'


