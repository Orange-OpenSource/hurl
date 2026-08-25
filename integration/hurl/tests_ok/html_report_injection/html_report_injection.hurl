GET http://localhost:8000/evil-headers
X-Bar: <script>alert('Hello')</script>
HTTP 200
