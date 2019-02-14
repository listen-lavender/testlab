#!/usr/bin/env python
# coding=utf8
import random
import datetime
import json
from flask import request, Blueprint
from models import Articles, Users, Comments

comment = Blueprint('comment', __name__)

@comment.route('/article/<aid>', methods=['GET'])
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

@comment.route('/user/<uid>', methods=['GET'])
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

@comment.route('', methods=['POST'])
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

@comment.route('/<cid>', methods=['PUT'])
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

@comment.route('/<cid>', methods=["DELETE"])
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


if __name__ == '__main__':
    pass

