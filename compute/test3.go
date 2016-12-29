package main

import (
    "fmt"
    "bytes"
    "strconv"
    // "strings"
    "runtime/debug"
    "github.com/coocood/freecache"
    "gopkg.in/mgo.v2"
    "gopkg.in/mgo.v2/bson"
    "encoding/json"
)

var buffer bytes.Buffer

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

func Fastjoin(separator string, args ...string) string {
    last := len(args) - 1
    if separator != "" {
        for k := 0; k < last; k++ {
            buffer.WriteString(args[k])
            buffer.WriteString(separator)
        }
    } else {
        for k := 0; k < last; k++ {
            buffer.WriteString(args[k])
        }
    }
    buffer.WriteString(args[last])
    str := buffer.String()
    buffer.Reset()
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

    cacheSize := 100 * 1024 * 1024
    cache := freecache.NewCache(cacheSize)
    debug.SetGCPercent(20)
    key := []byte(strconv.Itoa(user.User_id))
    val := JsonEncode(user)
    expire := 60 // expire in 60 seconds
    cache.Set(key, val, expire)
    got, err := cache.Get(key)
    if err != nil {
        fmt.Println(err)
    } else {
        fmt.Println(string(got))
    }
    affected := cache.Del(key)
    got_test, err_test := cache.Get([]byte("abcdefg"))
    if err_test != nil {
        fmt.Println("haha")
        fmt.Println(err_test)
    } else {
        fmt.Println(string(got_test))
    }
    fmt.Println("deleted key ", affected)
    fmt.Println("entry count ", cache.EntryCount())
}
