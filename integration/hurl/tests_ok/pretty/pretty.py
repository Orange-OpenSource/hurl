from app import app
from flask import make_response


@app.route("/pretty/demo")
def demo():
    data = """{"strings":{"english":"Hello, world!","chinese":"你好，世界","japanese":"こんにちは世界","korean":"안녕하세요 세계","arabic":"مرحبا بالعالم","hindi":"नमस्ते दुनिया","russian":"Привет, мир","greek":"Γειά σου Κόσμε","hebrew":"שלום עולם","accented":"Curaçao, naïve, façade, jalapeño"},"numbers":{"zero":0,"positive_int":42,"negative_int":-42,"large_int":1234567890123456789,"small_float":0.000123,"negative_float":-3.14159,"large_float":1.7976931348623157e308,"smallest_float":5e-324,"sci_notation_positive":6.022e23,"sci_notation_negative":-2.99792458e8},"booleans":{"isActive":true,"isDeleted":false},"emojis":{"happy":"😀","sad":"😢","fire":"🔥","rocket":"🚀","earth":"🌍","heart":"❤️","multi":"👩‍💻🧑🏽‍🚀👨‍👩‍👧‍👦"},"nothing":null}"""
    resp = make_response(data)
    resp.headers["Content-Type"] = "application/json"
    return resp
