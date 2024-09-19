POST http://localhost:8000/form-params
[FormParams]
param1: value1
param2:
param3: a=b
param4: a%3db
values\u{5b}0\u{5d}: 0
values[1]: 1
HTTP 200


POST http://localhost:8000/form-params
[Form]
param1: value1
param2:
param3: a=b
param4: a%3db
values\u{5b}0\u{5d}: 0
values[1]: 1
HTTP 200


# same version as raw
POST http://localhost:8000/form-params
Content-Type: application/x-www-form-urlencoded
`param1=value1&param2=&param3=a%3db&param4=a%253db&values[0]=0&values[1]=1`
HTTP 200
