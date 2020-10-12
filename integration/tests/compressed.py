from tests import app
from flask import make_response
from io import BytesIO

@app.route("/compressed/gzip")
def compressed_gzip():
    result = BytesIO()
    # echo -n 'Hello World!' | gzip -f | hexdump -C
    #result.write(b'\x1f\x8b\x08\x00\x0e\x2e\x83\x5f\x00\x03\xf3\x48\xcd\xc9\xc9\x57\x08\xcf\x2f\xca\x49\x51\xe4\x02\x00\xdd\xdd\x14\x7d\x0d')
    #                1f 8b 08 00 ed 0c 84 5f 00 03 f3 48 cd c9 c9 57 08 cf 2f ca 49 51 04 00  a3 1c 29 1c 0c 00 00 00
    result.write(b'\x1f\x8b\x08\x00\xed\x0c\x84\x5f\x00\x03\xf3\x48\xcd\xc9\xc9\x57\x08\xcf\x2f\xca\x49\x51\x04\x00\xa3\x1c\x29\x1c\x0c\x00\x00\x00')

    data = result.getvalue()
    resp = make_response(data)
    resp.headers['Content-Encoding'] = 'gzip'
    return resp

@app.route("/compressed/none")
def compressed_none():
    return 'Hello World!'