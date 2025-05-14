from app import app
from flask import request


@app.route("/parallel/hello")
def parallel_hello():
    name = request.args.get("name")
    return f"Hello {name} from a parallel world!\n"
