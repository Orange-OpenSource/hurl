from tests import app

@app.route('/put', methods=['PUT'])
def put():
    return ''