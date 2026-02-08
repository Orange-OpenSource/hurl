GET http://localhost:8000/deprecated
[QueryStringParams]     # Use [Query] instead
[FormParams]            # Use [Form] instead
[MultipartFormData]     # Use [Multipart] instead
HTTP 200
[Captures]
my_body: bytes decode "utf-8"
[Asserts]
header "x-custom-header" includes "value2" # Use contains instead
bytes decode "utf-8" == "café\n"
