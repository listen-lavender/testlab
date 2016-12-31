package compute

import (
	"gopkg.in/mgo.v2"
	"gopkg.in/mgo.v2/bson"
	"strconv"
)

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

func NewUserTop(spec bson.M) *UserTop {
	userTop := UserTop{}
	userTopCo.Find(spec).One(&userTop)
	if userTop.User_id == 0 {
		return nil
	}
	return &userTop
}

func HitUserTop(userid string) float64 {
	var userTop *UserTop
	name := Fastjoin("", userid, "top")
	result := GetCache(name)
	if result == nil {
		uid, _ := strconv.Atoi(userid)
		userTop = NewUserTop(bson.M{"User_id": uid})
		if userTop == nil {
			userTop = &UserTop{}
		} else {
			SetCache(name, userTop, 60*3)
		}
	} else {
		usertop, _ := result.(UserTop)
		userTop = &usertop
	}
	return userTop.Recommend_wight
}

func NewUserIdentity(spec bson.M) *UserIdentity {
	userIdentity := UserIdentity{}
	userIdentityCo.Find(spec).One(&userIdentity)
	if userIdentity.User_id == 0 {
		return nil
	}
	return &userIdentity
}

func HitUserIdentity(userid string) string {
	var userIdentity *UserIdentity
	name := Fastjoin("", userid, "idt")
	result := GetCache(name)
	if result == nil {
		uid, _ := strconv.Atoi(userid)
		userIdentity = NewUserIdentity(bson.M{"User_id": uid})
		if userIdentity == nil {
			userIdentity = &UserIdentity{}
		} else {
			SetCache(name, userIdentity, 60*10)
		}
	} else {
		useridentity, _ := result.(UserIdentity)
		userIdentity = &useridentity
	}
	return userIdentity.Recommend_level
}

func NewUser(spec bson.M) *User {
	user := &User{}
	userCo.Find(spec).One(user)
	// if user.User_id > 0{
	//     userTop := NewUserTop(bson.M{"User_id": user.User_id})
	//     user.Recommend_wight = userIdentity.Recommend_wight
	//     userIdentity := NewUserIdentity(bson.M{"User_id": user.User_id})
	//     user.Recommend_level = userIdentity.Recommend_level
	// }else{
	//     return nil
	// }
	if user.User_id == 0 {
		return nil
	}
	return user
}

func HitUser(userid string) *User {
	var user *User
	result := GetCache(userid)
	if result == nil {
		uid, _ := strconv.Atoi(userid)
		user = NewUser(bson.M{"User_id": uid})
		if user == nil {
			return user
		}
		SetCache(userid, user, 3600*8)
	} else {
		u, _ := result.(User)
		user = &u
	}
	return user
}

func NewLive(spec bson.M) *Live {
	live := &Live{}
	liveCo.Find(spec).One(live)
	if live.Live_id == 0 {
		return nil
	}
	return live
}

func HitLive(roomid string, belong []string) (*Live, float64, float64, map[string]float64, bool) {
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
		live = NewLive(bson.M{"room_id": rid})
		if !(live == nil) {
			if live.Country == "" {
				live.Country = "TH"
			}
			limitFactor, verifiedFactor, recommendFactor, unverified, onlineFactor, likeFactor, giftFactor = LiveFactor(live.Country)
			live.Extra = ComputeExtra(roomid, onlineFactor, likeFactor, giftFactor)
			SetCache(roomid, live, 60)
		}
	} else {
		l, _ := result.(Live)
		live = &l
	}
	if !(live == nil) {
		if len(belong) > 0 && IndexOf(live.Country, belong) == -1 {
			live = nil
		} else {
			if !(limitFactor == 0.0) {
				limitFactor, verifiedFactor, recommendFactor, unverified, _, _, _ = LiveFactor(live.Country)
			}
		}
	}
	return live, limitFactor, verifiedFactor, recommendFactor, unverified
}

func InitCfgList(spec bson.M, sortBy []string, limit int) {
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

func NewCfg(spec bson.M) *Cfg {
	cfg := &Cfg{}
	cfgCo.Find(spec).One(cfg)
	if cfg.Country == "" {
		return nil
	}
	return cfg
}

func HitCfg(country string) *Cfg {
	var cfg *Cfg
	result := GetCache(country)
	if result == nil {
		cfg = NewCfg(bson.M{"country": country})
		if cfg == nil {
			cfg = &Cfg{}
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
