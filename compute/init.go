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

const URL = "mongodb://user:passwd@host:port/admin"
const ADDR = "host:port"
const PWD = "pwd"
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
var CACHE map[string][]byte
var DEBUG bool

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

	CACHE = make(map[string][]byte)
	DEBUG = false
	InitCfgList(nil, nil, 0)
}
