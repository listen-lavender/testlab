package compute

import (
	"bytes"
	"github.com/coocood/freecache"
	"gopkg.in/mgo.v2"
	"gopkg.in/redis.v5"
	"runtime/debug"
)

var buffer bytes.Buffer
var cacheSize int
var cachePool *freecache.Cache

const URL = "mongodb://kitty:e5lSUCuDmoZE25Tx5BdO@172.31.15.69:31007/admin"
const ADDR = "172.31.0.74:6379"
const PWD = "4229e03326a029b60d839ab0a063fe85d3849f45a9b908cd013f3808d602d06b"
const DB = 0
const HotRoomSet = "room:hot"
const NewRoomSet = "room:new"
const CountryRoomSet = "country:hot"

var userCo *mgo.Collection
var userIdentityCo *mgo.Collection
var userTopCo *mgo.Collection
var liveCo *mgo.Collection
var cfgCo *mgo.Collection

var client *redis.Client
var expire int
var defaultCfg *Cfg

func init() {
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

	client = redis.NewClient(&redis.Options{
		Addr:     ADDR,
		Password: PWD,
		DB:       DB,
	})

	InitCfgList(nil, nil, 0)
}
