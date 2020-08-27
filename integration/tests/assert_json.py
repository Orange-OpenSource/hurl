from tests import app
from flask import Response

@app.route("/assert-json")
def assert_json():
    return Response('''{
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