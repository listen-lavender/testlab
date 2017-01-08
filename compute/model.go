package compute

import (
	"gopkg.in/mgo.v2"
	"gopkg.in/mgo.v2/bson"
	"strconv"
)

type Packet struct {
	Index string
	Country string
	Weight float64
	Liveid string
}

type User struct {
	User_id  int
	Kitty_id int
	Verified bool
}

type UserIdentity struct {
	User_id         int
	Recommend_level string
}

type UserTop struct {
	User_id         int
	Recommend_wight float64
}

type Live struct {
	Live_id    int
	User_id    int
	Status     int
	Country    string
	Lang       string
	T_publish  float64
	Created_at float64
	Extra      float64
}

type Cfg struct {
	Country string
	Config  Config
}

type Config struct {
	Country          []string
	Exe_span         int
	Limit_factor     float64
	Recommend_factor map[string]float64
	Verified_factor  float64
	Onlinemen_factor float64
	User_like_factor float64
	Gift_factor      float64
	Unverified       bool
}

func NewUserTop(spec bson.M, db *mgo.Database) *UserTop {
	if DEBUG{
		defer trace("NewUserTop")()
	}
	userTop := UserTop{}
	userTopCo := db.C("userTop")
	userTopCo.Find(spec).One(&userTop)
	return &userTop
}

func HitUserTop(userid string, db *mgo.Database) float64 {
	if DEBUG{
		defer trace("HitUserTop")()
	}
	var userTop *UserTop
	name := Fastjoin("", userid, "top")
	result := GetCache(name)
	if result == nil {
		uid, _ := strconv.Atoi(userid)
		userTop = NewUserTop(bson.M{"user_id": uid}, db)
		if userTop.User_id > 0 {
			SetCache(name, userTop, 60*3)
		}
	} else {
		usertop, _ := result.(UserTop)
		userTop = &usertop
	}
	return userTop.Recommend_wight
}

func NewUserIdentity(spec bson.M, db *mgo.Database) *UserIdentity {
	if DEBUG{
		defer trace("NewUserIdentity")()
	}
	userIdentity := UserIdentity{}
	userIdentityCo := db.C("userIdentity")
	userIdentityCo.Find(spec).One(&userIdentity)
	return &userIdentity
}

func HitUserIdentity(userid string, db *mgo.Database) string {
	if DEBUG{
		defer trace("HitUserIdentity")()
	}
	var userIdentity *UserIdentity
	name := Fastjoin("", userid, "idt")
	result := GetCache(name)
	if result == nil {
		uid, _ := strconv.Atoi(userid)
		userIdentity = NewUserIdentity(bson.M{"user_id": uid}, db)
		if userIdentity.User_id > 0{
			SetCache(name, userIdentity, 60*10)
		}
	} else {
		useridentity, _ := result.(UserIdentity)
		userIdentity = &useridentity
	}
	return userIdentity.Recommend_level
}

func NewUser(spec bson.M, db *mgo.Database) *User {
	if DEBUG{
		defer trace("NewUser")()
	}
	user := &User{}
	userCo := db.C("user")
	userCo.Find(spec).One(user)
	// if user.User_id > 0{
	//     userTop := NewUserTop(bson.M{"user_id": user.User_id})
	//     user.Recommend_wight = userIdentity.Recommend_wight
	//     userIdentity := NewUserIdentity(bson.M{"user_id": user.User_id})
	//     user.Recommend_level = userIdentity.Recommend_level
	// }else{
	//     return nil
	// }
	return user
}

func HitUser(userid string, db *mgo.Database) *User {
	if DEBUG{
		defer trace("HitUser")()
	}
	var user *User
	result := GetCache(userid)
	if result == nil {
		uid, _ := strconv.Atoi(userid)
		user = NewUser(bson.M{"user_id": uid}, db)
		if user.User_id > 0 {
			SetCache(userid, user, 3600*8)
		}
	} else {
		u, _ := result.(User)
		user = &u
	}
	return user
}

func NewLive(spec bson.M, db *mgo.Database) *Live {
	if DEBUG{
		defer trace("NewLive")()
	}
	live := &Live{}
	liveCo := db.C("live")
	liveCo.Find(spec).One(live)
	if !(live.Status == 1){
		live.Live_id = 0
	}
	return live
}

func HitLive(roomid string, belong []string, db *mgo.Database) (*Live, float64, float64, map[string]float64, bool) {
	if DEBUG{
		defer trace("HitLive")()
	}
	var live *Live
	var limitFactor float64
	var verifiedFactor float64
	var recommendFactor map[string]float64
	var unverified bool
	var onlineFactor float64
	var likeFactor float64
	var giftFactor float64

	result := GetCache(roomid)
	if result == nil {
		rid, _ := strconv.Atoi(roomid)
		live = NewLive(bson.M{"room_id": rid}, db)
		if live.Live_id > 0 {
			if live.Country == "" {
				live.Country = "TH"
			}
			limitFactor, verifiedFactor, recommendFactor, unverified, onlineFactor, likeFactor, giftFactor = LiveFactor(live.Country, db)
			live.Extra = ComputeExtra(roomid, onlineFactor, likeFactor, giftFactor)
			SetCache(roomid, live, 60)
		}
	} else {
		l, _ := result.(Live)
		live = &l
	}
	if live.Live_id > 0 {
		if len(belong) > 0 && IndexOf(live.Country, belong) == -1 {
			live.Live_id = 0
		} else {
			if !(limitFactor == 0.0) {
				limitFactor, verifiedFactor, recommendFactor, unverified, _, _, _ = LiveFactor(live.Country, db)
			}
		}
	}
	return live, limitFactor, verifiedFactor, recommendFactor, unverified
}

func InitCfgList(spec bson.M, sortBy []string, limit int) {
	if DEBUG{
		defer trace("InitCfgList")()
	}
	lsession := gsession.Clone()
	defer lsession.Close()

	db := lsession.DB("kittylive")
	cfgCo := db.C("hotcfg")
	var iterCfg *mgo.Iter
	if limit == 0 && (sortBy == nil || len(sortBy) == 0) {
		iterCfg = cfgCo.Find(spec).Iter()
	} else if limit == 0 {
		iterCfg = cfgCo.Find(spec).Sort(Fastjoin(",", sortBy...)).Iter()
	} else if sortBy == nil || len(sortBy) == 0 {
		iterCfg = cfgCo.Find(spec).Limit(limit).Iter()
	} else {
		iterCfg = cfgCo.Find(spec).Limit(limit).Sort(Fastjoin(",", sortBy...)).Iter()
	}

	cfg := Cfg{}
	for iterCfg.Next(&cfg) {
		if cfg.Country == "" {
			continue
		}
		if cfg.Country == "DEFAULT" {
			defaultCfg = &cfg
			continue
		}
		SetCache(cfg.Country, cfg, 3600*12)
	}
}

func NewCfg(spec bson.M, db *mgo.Database) *Cfg {
	if DEBUG{
		defer trace("NewCfg")()
	}
	cfg := &Cfg{}
	cfgCo := db.C("hotcfg")
	cfgCo.Find(spec).One(cfg)
	return cfg
}

func HitCfg(country string, db *mgo.Database) *Cfg {
	if DEBUG{
		defer trace("HitCfg")()
	}
	var cfg *Cfg
	result := GetCache(country)
	if result == nil {
		cfg = NewCfg(bson.M{"country": country}, db)
		if cfg.Country == "" {
			cfg.Country = country
			cfg.Config = defaultCfg.Config
		}
		SetCache(cfg.Country, cfg, 3600*12)
	} else {
		c, _ := result.(Cfg)
		cfg = &c
	}
	return cfg
}

func GetCfg(country string) *Cfg {
    lsession := gsession.Clone()
	defer lsession.Close()

	db := lsession.DB("kittylive")
	return HitCfg(country, db)
}
