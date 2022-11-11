# This file fails only when the output has been compressed (--compressed) and is used 
#
# For example, with --test, the file does not fail as the output is not used/decompressed
# Similarly, the file is green in the test HTML report
GET http://localhost:8000/error-output-decompress
HTTP 200

