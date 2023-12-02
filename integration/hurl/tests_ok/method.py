from app import app


@app.route("/methods/get")
def method_get():
    return ""


@app.route("/methods/head")
def method_head():
    return ""


@app.route("/methods/post", methods=["POST"])
def method_post():
    return ""


@app.route("/methods/put", methods=["PUT"])
def method_put():
    return ""


@app.route("/methods/delete", methods=["DELETE"])
def method_delete():
    return ""


@app.route("/methods/connect", methods=["CONNECT"])
def method_connect():
    return ""


@app.route("/methods/options", methods=["OPTIONS"])
def method_options():
    return ""


@app.route("/methods/trace", methods=["TRACE"])
def method_trace():
    return ""


@app.route("/methods/patch", methods=["PATCH"])
def method_patch():
    return ""


@app.route("/methods/link", methods=["LINK"])
def method_link():
    return ""


@app.route("/methods/unlink", methods=["UNLINK"])
def method_unlink():
    return ""


@app.route("/methods/purge", methods=["PURGE"])
def method_purge():
    return ""


@app.route("/methods/lock", methods=["LOCK"])
def method_lock():
    return ""


@app.route("/methods/unlock", methods=["UNLOCK"])
def method_unlock():
    return ""


@app.route("/methods/propfind", methods=["PROPFIND"])
def method_propfind():
    return ""


@app.route("/methods/view", methods=["VIEW"])
def method_view():
    return ""
