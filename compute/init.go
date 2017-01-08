package compute

import (
	"github.com/coocood/freecache"
	"gopkg.in/mgo.v2"
	"gopkg.in/redis.v5"
	"runtime/debug"
	// "time"
)

var cacheSize int
var cachePool *freecache.Cache

const URL = "mongodb://user:passwd@host:port/authdb"
const ADDR = "host:port"
const PWD = "pwd"
const DB = 0
const HotRoomSet = "room:hot"
const NewRoomSet = "room:new"
const CountryRoomSet = "country:hot"

var gsession *mgo.Session
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

    var err error
	gsession, err = mgo.Dial(URL)
	// gsession, err = mgo.DialWithInfo(&mgo.DialInfo{
	// 	Addrs:    []string{"host:port"},
	// 	Timeout:  60 * time.Second,
	// 	Database: "authdb",
	// 	Username: "user",
	// 	Password: "passwd",
	// })

	if err != nil {
		panic(err)
	}
	// defer gsession.Close()
	gsession.SetPoolLimit(100)
	gsession.SetMode(mgo.Monotonic, true)

	client = redis.NewClient(&redis.Options{
		Addr:     ADDR,
		Password: PWD,
		DB:       DB,
	})

	CACHE = make(map[string][]byte)
	DEBUG = false
	InitCfgList(nil, nil, 0)
}
