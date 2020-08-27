from tests import app
from flask import Response

@app.route("/assert-match")
def assert_match():
    return Response('''{
  "date1": "2014-01-01",
  "date2": "x2014-01-01"
}''', mimetype='application/json')


