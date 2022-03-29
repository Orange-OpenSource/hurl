from app import app, load_tests
from werkzeug.serving import WSGIRequestHandler

if __name__ == "__main__":
    load_tests()
    WSGIRequestHandler.protocol_version = "HTTP/1.0"
    app.run(host="0.0.0.0", port=8000)
