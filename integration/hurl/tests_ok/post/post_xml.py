# coding=utf-8
from app import app
from flask import request


@app.route("/post-xml", methods=["POST"])
def post_xml():
    s = request.data.decode("utf-8")
    assert (
        s
        == """<?xml version="1.0"?>
<drink>caf\u00e9</drink>"""
    )
    return ""


@app.route("/post-xml-no-prolog", methods=["POST"])
def post_xml_no_prolog():
    s = request.data.decode("utf-8")
    assert s == """<drink>caf\u00e9</drink>"""
    return ""


@app.route("/post-xml-large", methods=["POST"])
def post_xml_large():
    s = request.data.decode("utf-8")
    assert len(s) == 22156
    return ""
