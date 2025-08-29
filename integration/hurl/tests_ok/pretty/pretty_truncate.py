from app import app
from flask import make_response


@app.route("/pretty/demo-truncate")
def demo_truncate():
    data = """{"strings":{"english":"Hello, world!","chinese":"你好，世界","japanese":"こんにちは世界","korean":"안녕하세요"""
    resp = make_response(data)
    resp.headers["Content-Type"] = "application/json"
    return resp
