from app import app


@app.route("/get-variables-not-renderable")
def get_variables_not_renderable():
    return '{"list":[1,2,3], "object":{"id":1}, "nodeset": "<node></node>"}'
