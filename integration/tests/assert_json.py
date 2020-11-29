from tests import app
from flask import Response

@app.route("/assert-json")
def assert_json():
    return Response('''{
  "count": 5,
  "success": false,
  "errors": [{"id":"error1"},{"id":"error2"}],
  "warnings": [],
  "duration": 1.5,
  "tags": ["test"],
  "nullable": null
}''', mimetype='application/json')


@app.route("/assert-json/index")
def assert_json_index():
    return "1"

@app.route("/assert-json/list")
def assert_json_list():
    return Response('''[
  { "id": 1, "name": "Bob"},
  { "id": 2, "name": "Bill"}
]''', mimetype='application/json')

