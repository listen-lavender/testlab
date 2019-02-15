````bash
$ pip install -r requirements.txt
$ cd micro_blog
$ python run.py
(使用postmen进行API测试，ctrl+c)
$ cd mvt_blog
$ python run.py
(使用postmen进行API测试，ctrl+c)
$ cd blueprint_blog
$ python run.py
(使用postmen进行API测试，ctrl+c)
````

get http://127.0.0.1:5000/article

get http://127.0.0.1:5000/article/1

post http://127.0.0.1:5000/article {"title":"问世间", "content":"问世间，情为何物，直教生死相许？天南地北双飞客，老翅几回寒暑。欢乐趣，离别苦，就中更有痴儿女。君应有语，渺万里层云，千山暮雪，只影向谁去？ 横汾路，寂寞当年箫鼓，荒烟依旧平楚。招魂楚些何嗟及，山鬼暗啼风雨。天也妒，未信与，莺儿燕子俱黄土。千秋万古，为留待骚人，狂歌痛饮，来访雁丘处。", "uid":2}

put http://127.0.0.1:5000/article/1 {"title":"我们都是好孩子", "content":"再到结婚生子、继而看着孩子一天天长大的满足与凄美的《对你说》，歌曲犹如牛奶般细腻浓郁，无不感人至深；期间在舒缓中穿插两首清新凉爽的快歌《", "uid":2}

delete http://127.0.0.1:5000/article/1 {"uid":2}

---

get http://127.0.0.1:5000/comment/article/2

get http://127.0.0.1:5000/comment/user/2

post http://127.0.0.1:5000/comment {"content":"滚蛋", "aid":3, "uid":2}

put http://127.0.0.1:5000/comment/1 {"content":"哈哈", "aid":3, "uid":2}

delete http://127.0.0.1:5000/comment/1 {"aid":3, "uid":2}

---

get http://127.0.0.1:5000/user

get http://127.0.0.1:5000/user/1

---

有一段flask蓝图课程视频，第一次录，多多包涵。