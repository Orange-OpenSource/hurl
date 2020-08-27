# coding=utf-8
from tests import app

@app.route("/utf8")
def utf8():
    return '<data>café</data>'