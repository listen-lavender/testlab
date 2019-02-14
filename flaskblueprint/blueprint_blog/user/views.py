#!/usr/bin/env python
# coding=utf8
import random
import datetime
import json
from flask import request, Blueprint
from models import Articles, Users, Comments

user = Blueprint('user', __name__)

@user.route('', methods=['GET'])
@user.route('/<uid>', methods=['GET'])
def get_user(uid=None):
    uid = int(uid or 0)
    if uid == 0:
        result = []
        for uid, u in Users.items():
            r = dict({}, **u)
            result.append(r)
        return json.dumps({'code':0, 'desc':'success', 'res':result}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    elif not uid in Users:
        return json.dumps({'code':1, 'desc':'fail', 'res':"找不到该用户了"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        r = dict({}, **Users[uid])
        return json.dumps({'code':0, 'desc':'success', 'res':r}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')


if __name__ == '__main__':
    pass

