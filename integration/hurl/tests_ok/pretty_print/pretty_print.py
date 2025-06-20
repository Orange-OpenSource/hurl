from app import app
from json import dumps

@app.route("/json-simple")
def json_simple():
    return dumps({"name": "John", "age": 30}), 200, {'Content-Type': 'application/json'}

@app.route("/json-complex")
def json_complex():
    return dumps({
        "users": [
            {"id": 1, "name": "Alice", "details": {"email": "alice@example.com", "active": True}},
            {"id": 2, "name": "Bob", "details": {"email": "bob@example.com", "active": False}}
        ],
        "total": 2
    }), 200, {'Content-Type': 'application/json'}

@app.route("/json-array")
def json_array():
    return dumps(["item1", "item2", "item3"]), 200, {'Content-Type': 'application/json'}

@app.route("/invalid-json")
def invalid_json():
    return '{"invalid":json,content}', 200, {'Content-Type': 'application/json'}
