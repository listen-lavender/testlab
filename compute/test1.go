package main

import (
	"fmt"
    "strings"
	"gopkg.in/mgo.v2"
	"gopkg.in/mgo.v2/bson"
    "encoding/json"
)

const URL = "mongodb://kitty:e5lSUCuDmoZE25Tx5BdO@172.31.15.69:31007/admin"

type User struct {
	User_id         int
	Kitty_id        int
	Verified        bool
	Recommend_level int
	Recommend_wight int
}

type UserIdentity struct {
	User_id         int
	Recommend_level int
}

type UserTop struct {
	User_id         int
	Recommend_wight int
}

type Live struct {
	Live_id    int
	User_id    int
	Country    string
	Lang       string
	T_publish  float64
	Created_at float64
}

type Cfg struct {
	Country string
	Config  interface{}
}

func JsonDecode(str []byte) map[string]interface{} {
    var dict map[string]interface{}
    err := json.Unmarshal(str, &dict)
    if err != nil {
        fmt.Println(err)
        return nil
    }
    return dict
}

func JsonEncode(dict interface{}) []byte {
    str, err := json.Marshal(dict)
    if err != nil {
        fmt.Println(err)
        return nil
    }
    return str
}

func main() {

	session, err := mgo.Dial(URL) //连接数据库
	if err != nil {
		panic(err)
	}
	defer session.Close()
	session.SetMode(mgo.Monotonic, true)

	db := session.DB("kittylive")
	userCo := db.C("user")
	liveCo := db.C("live")
	cfgCo := db.C("hotcfg")
	userIdentityCo := db.C("userIdentity")
	userTopCo := db.C("userTop")

	count, err := userCo.Count()
	if err != nil {
		panic(err)
	}
	fmt.Println("total user: ", count)

	user := User{}
	err = userCo.Find(bson.M{"nickname": "listen"}).One(&user)
	fmt.Println("User:", user.User_id, user.Kitty_id)

    userIdentity := UserIdentity{}
    err = userIdentityCo.Find(bson.M{"user_id": user.User_id}).One(&userIdentity)
    fmt.Println("User:", userIdentity.User_id, userIdentity.Recommend_level)

    userTop := UserTop{}
    err = userTopCo.Find(bson.M{"user_id": user.User_id}).One(&userTop)
    fmt.Println("User:", userTop.User_id, userTop.Recommend_wight)

	// var liveList Live
    sortBy := []string{"-user_id", "_id"}
	live := Live{}
	iterLive := liveCo.Find(bson.M{"status": 1}).Limit(10).Sort(strings.Join(sortBy, ",")).Iter()
	for iterLive.Next(&live) {
		fmt.Printf("Live:%v%v\n", live.Live_id, live.T_publish)
	}

    cfg := Cfg{}
    iterCfg := cfgCo.Find(nil).Iter()
    for iterCfg.Next(&cfg) {
        fmt.Printf("Cfg:%v\n", cfg.Country)
        Config := string(JsonEncode(cfg.Config))
        fmt.Printf("Cfg:%v\n", Config)
    }
}
