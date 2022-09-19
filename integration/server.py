from app import app, load_tests

if __name__ == "__main__":
    load_tests()
    app.run(host="127.0.0.1", port=8000)
