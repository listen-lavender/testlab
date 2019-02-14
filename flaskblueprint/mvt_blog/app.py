#!/usr/bin/env python
# coding=utf8
from gevent.pywsgi import WSGIServer
from views import app

if __name__ == '__main__':
    # Development
    app.run(debug=True, host="0.0.0.0", port="5000")
    
    # Production
    # http_server = WSGIServer(('0.0.0.0', 5000), app)
    # http_server.serve_forever()


