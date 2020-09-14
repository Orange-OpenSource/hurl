from tests import app
from flask import redirect

@app.route('/follow-redirect')
def follow_redirect():
    return redirect('http://localhost:8000/following-redirect')

@app.route('/following-redirect')
def following_redirect():
    return redirect('http://localhost:8000/followed-redirect')

@app.route('/followed-redirect')
def followed_redirect():
    return 'Followed redirect!'
