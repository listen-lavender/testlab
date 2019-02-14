#!/usr/bin/env python
# coding=utf8
from flask import Flask
from gevent.pywsgi import WSGIServer
from user.views import user
from comment.views import comment
from article.views import article

app = Flask(__name__)

app.register_blueprint(user, 
    url_prefix='/user')
app.register_blueprint(comment, 
    url_prefix='/comment')
app.register_blueprint(article, 
    url_prefix='/article')

if __name__ == '__main__':
    # Development
    app.run(debug=True, host="0.0.0.0", port="5000")
    
    # Production
    # http_server = WSGIServer(('0.0.0.0', 5000), app)
    # http_server.serve_forever()


