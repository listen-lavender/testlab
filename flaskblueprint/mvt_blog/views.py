#!/usr/bin/env python
# coding=utf8
import random
import datetime
import json
from flask import Flask, request
from models import Articles, Comments, Users

app = Flask(__name__)

@app.route('/article', methods=['GET'])
@app.route('/article/<aid>', methods=['GET'])
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

@app.route('/article', methods=['POST'])
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

@app.route('/article/<aid>', methods=['PUT'])
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

@app.route('/article/<aid>', methods=["DELETE"])
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


@app.route('/comment/article/<aid>', methods=['GET'])
def get_comment_by_aid(aid):
    aid = int(aid or 0)
    if aid in Articles:
        result = []
        for cid, c in Comments.items():
            if cid == "next" or not c["aid"] == aid or not c["status"] == 1:
                continue
            r = dict({}, **c)
            r["cid"] = cid
            r["user"] = Users[r["uid"]]
            result.append(r)
        return json.dumps({'code':0, 'desc':'success', 'res':result}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        return json.dumps({'code':1, 'desc':'fail', 'res':"找不到该文章了"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')

@app.route('/comment/user/<uid>', methods=['GET'])
def get_comment_by_uid(uid):
    uid = int(uid or 0)
    if uid in Users:
        result = []
        for cid, c in Comments.items():
            if cid == "next" or not c["uid"] == uid:
                continue
            r = dict({}, **c)
            r["cid"] = cid
            r["article_ref"] = "/article/" + str(c["aid"])
            result.append(r)
        return json.dumps({'code':0, 'desc':'success', 'res':result}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        return json.dumps({'code':1, 'desc':'fail', 'res':"找不到该用户了"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')

@app.route('/comment', methods=['POST'])
def post_comment():
    aid = int(request.form.get('aid') or 0)
    uid = int(request.form.get('uid') or 0)
    content = request.form.get('content', '')
    if content == "" or not uid in Users or not aid in Articles:
        return json.dumps({'code':1, 'desc':'fail', 'res':"添加评论失败"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        cid = Comments["next"]
        create = (datetime.datetime.now() + datetime.timedelta(days=random.randint(1, 10))).strftime("%Y-%m-%d")
        Comments[cid]= {
            "content":content,
            "aid":aid,
            "uid":uid,
            "status":1,
            "create":create,
        }
        Comments["next"] = Comments["next"] + 1
        return json.dumps({'code':0, 'desc':'success', 'res':"添加评论成功"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')

@app.route('/comment/<cid>', methods=['PUT'])
def put_comment(cid):
    cid = int(cid or 0)
    aid = int(request.form.get('aid') or 0)
    uid = int(request.form.get('uid') or 0)
    content = request.form.get('content', '')
    if not cid in Comments or not Comments[cid]["status"] == 1 or not aid == Comments[cid]["aid"] or not uid == Comments[cid]["uid"]:
        return json.dumps({'code':1, 'desc':'fail', 'res':"找不到该评论了"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        try:
            create = (datetime.datetime.now() + datetime.timedelta(days=random.randint(1, 10))).strftime("%Y-%m-%d")
            Comments[cid]["content"] = content or Comments[aid]["content"]
            Comments[cid]["create"] = create
            return json.dumps({'code':0, 'desc':'success', 'res':"修改评论成功"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
        except:
            return json.dumps({'code':1, 'desc':'fail', 'res':"修改评论失败"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')

@app.route('/comment/<cid>', methods=["DELETE"])
def delete_comment(cid):
    cid = int(cid or 0)
    aid = int(request.form.get('aid') or 0)
    uid = int(request.form.get('uid') or 0)
    if not cid in Comments or not Comments[cid]["status"] == 1 or not aid == Comments[cid]["aid"] or not uid == Comments[cid]["uid"]:
        return json.dumps({'code':1, 'desc':'fail', 'res':"找不到该评论了"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
    else:
        try:
            Comments[cid]["status"] = 0
            return json.dumps({'code':0, 'desc':'success', 'res':"删除评论成功"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')
        except:
            return json.dumps({'code':1, 'desc':'fail', 'res':"删除评论失败"}, ensure_ascii=False, sort_keys=True, indent=4).encode('utf8')


@app.route('/user', methods=['GET'])
@app.route('/user/<uid>', methods=['GET'])
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

