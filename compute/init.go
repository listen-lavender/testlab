package compute

import (
    "bytes"
    "encoding/json"
    "github.com/coocood/freecache"
    "fmt"
    "runtime/debug"
    "strconv"
    "strings"
)

var buffer bytes.Buffer
var cacheSize int
var cachePool *Cache

const URL = "mongodb://kitty:e5lSUCuDmoZE25Tx5BdO@172.31.15.69:31007/admin"
var userCo *Collection
var userIdentityCo *Collection
var userTopCo *Collection
var liveCo *Collection
var cfgCo *Collection

var client *Client
var activeRoomSet string
var hotRoomSet string
var newRoomSet string
var countryRoomSet string
var expire int

var defaultCfg *Cfg

func init(){
    cacheSize = 100 * 1024 * 1024
    cachePool = freecache.NewCache(cacheSize)
    debug.SetGCPercent(20)
    expire = 20

    session, err := mgo.Dial(URL)
    if err != nil {
        panic(err)
    }
    // defer session.Close()
    session.SetMode(mgo.Monotonic, true)

    db := session.DB("kittylive")
    userCo = db.C("user")
    userIdentityCo = db.C("userIdentity")
    userTopCo = db.C("userTop")
    liveCo = db.C("live")
    cfgCo = db.C("hotcfg")

    activeRoomSet := "room:active"
    hotRoomSet := "room:hot"
    newRoomSet := "room:new"
    countryRoomSet := "country:hot"
    client = redis.NewClient(&redis.Options{
        Addr:     "172.31.0.74:6379",
        Password: "4229e03326a029b60d839ab0a063fe85d3849f45a9b908cd013f3808d602d06b", // no password set
        DB:       0,
    })

    InitCfgList()
}
