GET http://localhost:8000/evil-headers
X-Bar: <script>alert('Hello')</script>
HTTP 200
[Captures]
\u{3c}script\u{3e}alert\u{28}\u{22}Hello\u{22}\u{29}\u{3c}\u{2f}script\u{3e}: status
