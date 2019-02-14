#!/usr/bin/env python
# coding=utf8
import random
import datetime
import json
from flask import request, Blueprint
from models import Articles, Users, Comments

article = Blueprint('article', __name__)

@article.route('', methods=['GET'])
@article.route('/<aid>', methods=['GET'])
def get_article(aid=None):
    aid = int(aid or 0)
    if aid == 0:
        result = []
        for aid, a in Articles.items():
            if aid == "next" or not a["status"] == 1:
                continue
            r = dict({}, **a)
            r["user"] = Users[r["uid"]]
            r["aid"] = aid
            result.append(r)
        return json.dumps({'code':0, 'desc':'success', 'res':result}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    elif not aid in Articles:
        return json.dumps({'code':1, 'desc':'fail', 'res':"找不到该文章了"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        r = dict({}, **Articles[aid])
        r["user"] = Users[r["uid"]]
        return json.dumps({'code':0, 'desc':'success', 'res':r}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')


@article.route('', methods=['POST'])
def post_article():
    title = request.form.get('title', '')
    content = request.form.get('content', '')
    uid = int(request.form.get('uid') or 0)
    if title == "" or content == "" or not uid in Users:
        return json.dumps({'code':1, 'desc':'fail', 'res':"添加文章失败"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        aid = Articles["next"]
        create = (datetime.datetime.now() + datetime.timedelta(days=random.randint(1, 10))).strftime("%Y-%m-%d")
        update = create
        Articles[aid]= {
            "title":title,
            "content":content,
            "uid":uid,
            "status":1,
            "update":update,
            "create":create,
        }
        Articles["next"] = Articles["next"] + 1
        return json.dumps({'code':0, 'desc':'success', 'res':"添加文章成功"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')


@article.route('/<aid>', methods=['PUT'])
def put_article(aid):
    aid = int(aid or 0)
    uid = int(request.form.get('uid') or 0)
    if not aid in Articles or not Articles[aid]["status"] == 1 or not Articles[aid]["uid"] == uid:
        return json.dumps({'code':1, 'desc':'fail', 'res':"找不到该文章了"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        try:
            title = request.form.get('title', '')
            content = request.form.get('content', '')
            update = (datetime.datetime.now() + datetime.timedelta(days=random.randint(1, 10))).strftime("%Y-%m-%d")
            Articles[aid]["title"] = title or Articles[aid]["title"]
            Articles[aid]["content"] = content or Articles[aid]["content"]
            Articles[aid]["update"] = update
            return json.dumps({'code':0, 'desc':'success', 'res':"修改文章成功"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
        except:
            return json.dumps({'code':1, 'desc':'fail', 'res':"修改文章失败"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')

@article.route('/<aid>', methods=["DELETE"])
def delete_article(aid):
    aid = int(aid or 0)
    uid = int(request.form.get('uid') or 0)
    if not aid in Articles or not Articles[aid]["status"] == 1 or not Articles[aid]["uid"] == uid:
        return json.dumps({'code':1, 'desc':'fail', 'res':"找不到该文章了"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        try:
            Articles[aid]["status"] = 0
            return json.dumps({'code':0, 'desc':'success', 'res':"删除章成功"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
        except:
            return json.dumps({'code':1, 'desc':'fail', 'res':"删除文章失败"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')


if __name__ == '__main__':
    pass


