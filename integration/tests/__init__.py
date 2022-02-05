from flask import Flask
import glob
import importlib
import os

app = Flask(__name__)

current_dir = os.path.basename(os.path.dirname(__file__))
for python_file in glob.glob(current_dir + "/*.py"):
    if python_file.endswith("__init__.py"):
        continue
    module_name = python_file.split(".")[0].replace(os.path.sep, ".")
    print("loading %s" % module_name)
    try:
        importlib.import_module(module_name)
    except ImportError as err:
        print("Error:", err)


@app.after_request
def remove_header(response):
    response.headers["Server"] = "Flask Server"
    return response
