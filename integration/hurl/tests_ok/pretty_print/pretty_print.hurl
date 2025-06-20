# Test pretty-print functionality with JSON responses

# Test 1: Simple JSON object
GET http://localhost:8000/json-simple
HTTP 200
Content-Type: application/json

# Test 2: Complex nested JSON
GET http://localhost:8000/json-complex
HTTP 200
Content-Type: application/json

# Test 3: JSON array
GET http://localhost:8000/json-array
HTTP 200
Content-Type: application/json

# Test 4: Non-JSON content (should remain unchanged)
GET http://localhost:8000/hello
HTTP 200
Content-Type: text/html; charset=utf-8

# Test 5: Invalid JSON (should remain unchanged)
GET http://localhost:8000/invalid-json
HTTP 200
Content-Type: application/json
