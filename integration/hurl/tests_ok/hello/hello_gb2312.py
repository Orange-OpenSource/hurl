from app import app
from flask import Response


@app.route("/hello_gb2312")
def hello_gb2312():
    headers = {"Content-Type": "text/html; charset=gb2312"}
    data = """<!DOCTYPE html>
<html>
    <head>
        <meta http-equiv='Content-Type' content='text/html; charset=gb2312'>
    </head>
    <body>你好世界</body>
</html>
""".encode("gb2312")
    return Response(data, headers=headers)


@app.route("/hello_gb2312_implicit")
def hello_gb2312_implicit():
    headers = {"Content-Type": "text/html"}
    data = """<!DOCTYPE html>
<html>
    <head>
        <meta http-equiv='Content-Type' content='text/html; charset=gb2312'>
    </head>
    <body>你好世界</body>
</html>
""".encode("gb2312")
    return Response(data, headers=headers)
