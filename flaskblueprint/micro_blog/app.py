#!/usr/bin/env python
# coding=utf8
import random
import datetime
import json
from flask import Flask, request
from gevent.pywsgi import WSGIServer

app = Flask(__name__)

Articles = {
    1:{
        "title":"亚当和夏娃",
        "content":"亚当和夏娃是《圣经》里的人物。据说，神·耶和华用五天时间创造了天地万物，第六天，用尘土创造了亚当。亚当是世上第一个人类和第一个男人，后来神·耶和华又用亚当的一根肋骨创造了第一个女人——夏娃，并让他们结为夫妻，共同生活在伊甸园。后来夏娃受蛇的诱惑，偷食了善恶树的禁果，并让亚当食用。神·耶和华发现后，对亚当和夏娃进行了惩罚，把二人逐出了伊甸园，二人成为人类的祖先。",
        "uid":2,
        "status":1,
        "update":"2018-12-31",
        "create":"2018-12-11",
    },
    2:{
        "title":"冰与火之歌",
        "content":"故事从维斯特洛大陆边境处发现远古传说中早已灭绝的生物开始，预示着危险即将到来。而这片大陆的临冬城主暨北境统领艾德·史塔克的家族也迎来了老友兼国王劳勃·拜拉席恩的来访。国王希望艾德·史塔克能担任首相一职，对抗企图夺取铁王座的叛军。",
        "uid":2,
        "status":1,
        "update":"2018-12-21",
        "create":"2018-11-24",
    },
    3:{
        "title":"春节",
        "content":"春节，即农历新年，是一年之岁首，传统意义上的“年节”。俗称新春、新岁、新年、新禧、年禧、大年等，口头上又称度岁、庆岁、过年、过大年。春节历史悠久，由上古时代岁首祈年祭祀演变而来。",
        "uid":1,
        "status":1,
        "update":"2018-11-17",
        "create":"2018-01-24",
    },
    "next":4,
}

Users = {
    1:{
        "name":"美少女",
        "tel":"18722392278",
        "email":"848565999@qq.com",
    },
    2:{
        "name":"旋风小子",
        "tel":"17745395565",
        "email":"445465343@qq.com",
    },
}

Comments = {
    1:{
        "aid":1,
        "uid":1,
        "content":"扯淡",
        "create":"2018-12-12",
        "status":1,
    },
    2:{
        "aid":1,
        "uid":2,
        "content":"好看",
        "create":"2018-12-12",
        "status":1,
    },
    3:{
        "aid":2,
        "uid":2,
        "content":"菜鸟，写的差",
        "create":"2018-12-12",
        "status":1,
    },
    4:{
        "aid":3,
        "uid":2,
        "content":"越来越好",
        "create":"2018-12-12",
        "status":1,
    },
    5:{
        "aid":3,
        "uid":2,
        "content":"改改吧",
        "create":"2018-12-12",
        "status":1,
    },
    6:{
        "aid":2,
        "uid":1,
        "content":"不可能",
        "create":"2018-12-12",
        "status":1,
    },
    "next":7,
}

@app.route('/article', methods=['GET']) # 获取博客文章列表
@app.route('/article/<aid>', methods=['GET']) # 获取某一篇博客文章
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

@app.route('/article', methods=['POST']) # 发布一篇博客
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

@app.route('/article/<aid>', methods=['PUT']) # 修改一篇博客
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

@app.route('/article/<aid>', methods=["DELETE"]) # 删除一篇博客
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


@app.route('/comment/article/<aid>', methods=['GET']) # 获取一篇博客的评论列表
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

@app.route('/comment/user/<uid>', methods=['GET']) # 获取一个用户的评论列表
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

@app.route('/comment', methods=['POST']) # 发布一个评论
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

@app.route('/comment/<cid>', methods=['PUT']) # 修改一个评论
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

@app.route('/comment/<cid>', methods=["DELETE"]) # 删除一个评论
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


@app.route('/user', methods=['GET']) # 获取用户列表
@app.route('/user/<uid>', methods=['GET']) # 获取某个用户信息
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
    # Development
    app.run(debug=True, host="0.0.0.0", port="5000")
    
    # Production
    # http_server = WSGIServer(('0.0.0.0', 5000), app)
    # http_server.serve_forever()


