from app import app
from flask import request


@app.route("/multilines/plain-text", methods=["POST"])
def multilines_plain_text():
    expected_body = "line1\nline2\nline3\n"
    body_in = request.data.decode("utf-8")
    assert expected_body == body_in
    return expected_body


@app.route("/multilines/json", methods=["POST"])
def multilines_json():
    expected_body = """\
{
  "foo": "bar"
  "baz": 123456
}
"""
    body_in = request.data.decode("utf-8")
    assert expected_body == body_in
    return expected_body


@app.route("/multilines/xml", methods=["POST"])
def multilines_xml():
    expected_body = """\
<?xml version="1.0"?>
<catalog>
    <book id="bk101">
        <author>Gambardella, Matthew</author>
        <title>XML Developer's Guide</title>
        <genre>Computer</genre>
        <price>44.95</price>
        <publish_date>2000-10-01</publish_date>
        <description>An in-depth look at creating applications
        with XML.</description>
    </book>
</catalog>
"""
    body_in = request.data.decode("utf-8")
    assert expected_body == body_in
    return expected_body


@app.route("/multilines/graphql", methods=["POST"])
def multilines_graphql():
    expected_body = r'{"query":"{\n  hero {\n    name\n    # Queries can have comments!\n    friends {\n      name\n    }\n  }\n}"}'
    body_in = request.data.decode("utf-8")
    assert expected_body == body_in
    return expected_body
