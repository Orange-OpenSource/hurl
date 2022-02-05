from flask import Flask
from flask import request

app = Flask(__name__)


@app.route("/hello")
def hello():
    return "Hello World!"


app.run(host="0.0.0.0", port=8000)
