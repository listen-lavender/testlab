use std::rc::Rc;
use std::mem::swap;
use std::cmp::max;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::cell::RefCell;
use std::cell::Cell;
use chrono::Utc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::thread;

struct Tweet {
    id: u32,
    ts: i64,
    content: String,
    next: Option<Rc<RefCell<Tweet>>>,
}

impl Ord for Tweet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ts.cmp(&other.ts).reverse()
    }
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other).reverse())
    }
}

impl Eq for Tweet {}

impl PartialEq for Tweet {
    fn eq(&self, other: &Self) -> bool {
        self.ts == other.ts
    }
}

impl Tweet {
    fn new(tweet_id:u32, ts:i64, content:String) -> Tweet {
        Tweet{
            id:tweet_id,
            ts:ts,
            content:content,
            next:None,
        }
    }
    fn next(&self) -> Option<Rc<RefCell<Tweet>>>{
        match &self.next {
            Some(next_node) => {
                Some(next_node.clone())
            }
            None => {
                None
            }
        }
        // let next_node = self.next.take();
        // if next_node.is_some() {
        //     let next_node = next_node.unwrap();
        //     self.next = Some(next_node.clone());
        //     Some(next_node)
        // } else {
        //     None
        // }
    }
    fn clone(&self) -> Tweet {
        Tweet::new(self.id, self.ts, self.content.clone())
    }
}

struct User {
    id: u32,
    name: String,
    followed: HashSet<u32>,
    tweet: Option<Rc<RefCell<Tweet>>>,
}

impl User {
    fn new(user_id:u32, name:String) -> User {
        User{
            id:user_id,
            name:name,
            followed:HashSet::new(),
            tweet:None,
        }
    }
    fn follow(&mut self, user_id:u32) {
        self.followed.insert(user_id);
    }
    fn unfollow(&mut self, user_id:u32) {
        self.followed.remove(&user_id);
    }
    fn post(&mut self, tweet_id:u32, ts:i64, content:String) {
        let mut content = "：".to_string() + &content;
        content = self.name.clone() + &content;
        let news = Tweet::new(tweet_id, ts, content);
        match &self.tweet {
            Some(tweet) => {
                let mut next_tweet = Some(tweet.clone());
                let mut last_tweet = tweet.clone();
                while let Some(tweet) = next_tweet {
                    last_tweet = tweet.clone();
                    next_tweet = tweet.borrow().next();
                }
                last_tweet.borrow_mut().next = Some(Rc::new(RefCell::new(news)));
            }
            None => {
                self.tweet = Some(Rc::new(RefCell::new(news)));
            }
        }
    }
    fn tweet(&self) -> Option<Rc<RefCell<Tweet>>>{
        match &self.tweet {
            Some(tweet) => {
                Some(tweet.clone())
            }
            None => {
                None
            }
        }
    }
}

struct Twitter {
    next_tweet_id: u32,
    next_user_id: u32,
    user_info: HashMap<u32, User>,
}

impl Twitter {
    fn new() -> Twitter {
        Twitter{
            next_tweet_id:0,
            next_user_id:0,
            user_info:HashMap::new(),
        }
    }
    fn get_user_id(&mut self) -> u32 {
        let user_id = self.next_user_id;
        self.next_user_id = self.next_user_id + 1;
        user_id
    }
    fn get_tweet_id(&mut self) -> u32 {
        let tweet_id = self.next_tweet_id;
        self.next_tweet_id = self.next_tweet_id + 1;
        tweet_id
    }

    fn register(&mut self, name: String) -> u32 {
        let user_id = self.get_user_id();
        let user = User::new(user_id, name);
        self.user_info.insert(user_id, user);
        user_id
    }

    fn postTweet(&mut self, user_id:u32, content:String) {
        let tweet_id = self.get_tweet_id();
        let user = self.user_info.get_mut(&user_id);
        match user {
            Some(user) => {
                let ts = Utc::now().timestamp();
                user.post(tweet_id, ts, content);
            }
            None => {}
        }
    }

    fn follow(&mut self, follower_id:u32, followed_id:u32) {
        let user = self.user_info.get_mut(&follower_id);
        match user {
            Some(user) => {
                user.follow(followed_id);
            }
            None => {}
        }
    }

    fn unfollow(&mut self, follower_id:u32, followed_id:u32) {
        let user = self.user_info.get_mut(&follower_id);
        match user {
            Some(user) => {
                user.unfollow(followed_id);
            }
            None => {}
        }
    }

    fn get_news(&self, user_id:u32) -> Option<Rc<RefCell<Tweet>>> {
        let user = self.user_info.get(&user_id);
        let mut news:Option<Rc<RefCell<Tweet>>> = None;
        match user {
            Some(user) => {
                let mut pq:BinaryHeap<Rc<RefCell<Tweet>>> = BinaryHeap::new();
                let mut all_followed = user.followed.clone();
                all_followed.insert(user_id);
                for followed_id in all_followed {
                    let followed = self.user_info.get(&followed_id);
                    if followed.is_none() {
                        continue
                    }

                    let followed = followed.unwrap();
                    let tweet = followed.tweet();
                    if tweet.is_none() {
                        continue
                    }

                    let tweet = tweet.unwrap();
                    let mut next_tweet = Some(tweet.clone());
                    while let Some(tweet) = next_tweet {
                        pq.push(Rc::new(RefCell::new(tweet.borrow().clone())));
                        next_tweet = tweet.borrow().next();
                    }
                }

                let tweet = pq.pop();
                if tweet.is_some() {
                    let mut prev = tweet.unwrap();
                    news = Some(prev.clone());
                    while let Some(tweet) = pq.pop() {
                        prev.borrow_mut().next = Some(tweet.clone());
                        prev = tweet.clone();
                    }
                }
            }
            None => {}
        }
        news
    }
}

fn test_twitter(mark: &str){
    println!("=====start {}", mark);
    let mut twitter = Twitter::new();

    let fuliye = twitter.register("傅立业".to_string());
    let zhouyu = twitter.register("周瑜".to_string());
    let judunhang = twitter.register("巨敦航".to_string());
    let wangluo = twitter.register("汪萝".to_string());
    let xinli = twitter.register("辛力".to_string());

    twitter.follow(zhouyu, fuliye);
    twitter.follow(judunhang, fuliye);
    twitter.follow(wangluo, fuliye);
    twitter.follow(xinli, fuliye);
    twitter.follow(zhouyu, wangluo);

    let yangyan = twitter.register("杨艳".to_string());
    let ayue = twitter.register("阿月".to_string());
    let zhouxingxing = twitter.register("周星星".to_string());
    let caikuo = twitter.register("蔡廓".to_string());
    let zhangguan = twitter.register("张关".to_string());
    let wumo = twitter.register("伍默".to_string());
    let fanxiang = twitter.register("范翔".to_string());
    let zengzihua = twitter.register("曾子画".to_string());

    twitter.follow(ayue, yangyan);
    twitter.follow(zhouxingxing, yangyan);
    twitter.follow(caikuo, yangyan);
    twitter.follow(zhangguan, yangyan);
    twitter.follow(wumo, yangyan);
    twitter.follow(fanxiang, yangyan);
    twitter.follow(zengzihua, yangyan);
    twitter.follow(zhouxingxing, ayue);
    twitter.follow(zhangguan, ayue);
    twitter.follow(zengzihua, ayue);
    twitter.follow(caikuo, wumo);

    twitter.follow(fuliye, yangyan);
    twitter.follow(ayue, fuliye);

    twitter.postTweet(fuliye, "找你喝酒".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(xinli, "真好玩".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(fuliye, "一起去溜达一下".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(wangluo, "我就想静静".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(fuliye, "心静自然凉".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(judunhang, "狂躁，我很狂躁".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(fuliye, "你还是守好城".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(zhouyu, "那里有很漂亮的衣服，我也想去".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(fuliye, "扬州一茬小姑娘长大了，我要去看看".to_string());
    thread::sleep_ms(1000);

    twitter.postTweet(yangyan, "子画最棒".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(wumo, "曾小姑娘加油".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(zhangguan, "曾小姑娘加油".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(caikuo, "曾小姑娘加油".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(zengzihua, "天下没有难做的生意".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(zhangguan, "讨厌智门".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(caikuo, "讨厌智门".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(yangyan, "你们的供奉没加".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(fanxiang, "智门左右天下大势".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(caikuo, "仁门的影响是春风化雨".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(yangyan, "仁门啥都没干".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(wumo, "仁门主持了4次放生".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(zhangguan, "我们比礼门的人生活艰苦，危险更多".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(zhangguan, "义门做了不少好事".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(yangyan, "义门伤员也不少".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(yangyan, "礼门经费变多了".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(caikuo, "礼门今年业绩辉煌".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(yangyan, "都要来".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(zhouxingxing, "阿月不去，我也不去".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(zhouxingxing, "开会要说什么".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(ayue, "我感冒了去不了".to_string());
    thread::sleep_ms(1000);
    twitter.postTweet(yangyan, "明天日月潭开大会".to_string());
    thread::sleep_ms(1000);

    println!("傅立业{}", mark);
    let mut next_tweet = twitter.get_news(fuliye);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("周瑜{}", mark);
    let mut next_tweet = twitter.get_news(zhouyu);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("巨敦航{}", mark);
    let mut next_tweet = twitter.get_news(judunhang);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("汪萝{}", mark);
    let mut next_tweet = twitter.get_news(wangluo);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("辛力{}", mark);
    let mut next_tweet = twitter.get_news(xinli);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");

    println!("杨艳{}", mark);
    let mut next_tweet = twitter.get_news(yangyan);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("阿月{}", mark);
    let mut next_tweet = twitter.get_news(ayue);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("周星星{}", mark);
    let mut next_tweet = twitter.get_news(zhouxingxing);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("蔡廓{}", mark);
    let mut next_tweet = twitter.get_news(caikuo);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("张关{}", mark);
    let mut next_tweet = twitter.get_news(zhangguan);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("伍默{}", mark);
    let mut next_tweet = twitter.get_news(wumo);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("范翔{}", mark);
    let mut next_tweet = twitter.get_news(fanxiang);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    println!("曾子画{}", mark);
    let mut next_tweet = twitter.get_news(zengzihua);
    while let Some(tweet) = next_tweet {
        println!("{}, {}", tweet.borrow().content, tweet.borrow().ts);
        next_tweet = tweet.borrow().next();
    }
    println!("");
    
    println!("=====end {}", mark);
}

fn main() {
    test_twitter("twitter");
}
