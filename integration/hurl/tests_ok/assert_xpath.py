# coding=utf-8
from flask import Response
from app import app


@app.route("/assert-xpath")
def assert_xpath():
    body = "<data>café</data>"
    return Response(body, mimetype="text/xml")


@app.route("/assert-xpath-svg")
def assert_xpath_svg():
    body = """<?xml version="1.0"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.0//EN" 
              "http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd">
<svg xmlns="http://www.w3.org/2000/svg">
  <style type="text/css">
    circle:hover {fill-opacity:0.9;}
  </style>
  <g style="fill-opacity:0.7;">
    <circle cx="6.5cm" cy="2cm" r="100" style="fill:red; stroke:black; stroke-width:0.1cm" transform="translate(0,50)" />
    <circle cx="6.5cm" cy="2cm" r="100" style="fill:blue; stroke:black; stroke-width:0.1cm" transform="translate(70,150)" />
    <circle cx="6.5cm" cy="2cm" r="100" style="fill:green; stroke:black; stroke-width:0.1cm" transform="translate(-70,150)"/>
  </g>
</svg>
"""
    return Response(body, mimetype="text/xml")


@app.route("/assert-xpath-simple-namespaces")
def assert_xpath_simple_ns():
    body = """<?xml version="1.0"?>
<!-- both namespace prefixes are available throughout -->
<bk:book xmlns:bk='urn:loc.gov:books'
         xmlns:isbn='urn:ISBN:0-395-36341-6'>
    <bk:title>Cheaper by the Dozen</bk:title>
    <isbn:number>1568491379</isbn:number>
</bk:book>
"""
    return Response(body, mimetype="text/xml")


@app.route("/assert-xpath-namespaces")
def assert_xpath_ns():
    body = """<?xml version="1.0"?>
<!-- initially, the default namespace is "books" -->
<book xmlns='urn:loc.gov:books'
      xmlns:isbn='urn:ISBN:0-395-36341-6'>
    <title>Cheaper by the Dozen</title>
    <isbn:number>1568491379</isbn:number>
    <notes>
      <!-- make HTML the default namespace for some commentary -->
      <p xmlns='http://www.w3.org/1999/xhtml'>
          This is a <i>funny</i> book!
      </p>
    </notes>
</book>
"""
    return Response(body, mimetype="text/xml")
