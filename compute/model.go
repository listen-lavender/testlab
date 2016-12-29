package compute

import (
    "fmt"
    "strings"
    "gopkg.in/mgo.v2"
    "gopkg.in/mgo.v2/bson"
    "encoding/json"
)

type User struct {
    User_id         int
    Kitty_id        int
    Verified        bool
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

func NewUserTop(spec M)*UserTop{
    userTop := UserTop{}
    userTopCo.Find(spec).One(&userTop)
    if userTop.User_id == 0{
        return nil
    }
    return &userTop
}

func HitUserTop(userid string) *UserTop{
    var userTop *UserTop
    name := Fastjoin("", userid, "top")
    result := GetCache(name)
    if result == nil{
        userTop = NewUserTop(bson.M{"User_id": strconv.Atoi(userid)})
        if userTop == nil{
            userTop = &UserTop{}
        }else{
            SetCache(name, userTop, 60 * 3)
        }
    }else{
        userTop = result.(userTop)
    }
    return userTop
}

func NewUserIdentity(spec M)*UserIdentity{
    userIdentity := UserIdentity{}
    userIdentityCo.Find(spec).One(&userIdentity)
    if userIdentity.User_id == 0{
        return nil
    }
    return &userIdentity
}

func HitUserIdentity(userid string) *UserIdentity{
    var userIdentity *UserIdentity
    name := Fastjoin("", userid, "idt")
    result := GetCache(name)
    if result == nil{
        userIdentity = NewUserIdentity(bson.M{"User_id": strconv.Atoi(userid)})
        if userIdentity == nil{
            userIdentity = &UserIdentity{}
        }else{
            SetCache(name, userIdentity, 60 * 10)
        }
    }else{
        userIdentity = result.(userIdentity)
    }
    return userIdentity
}

func NewUser(spec M)*User{
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
    if user.User_id == 0{
        return nil
    }
    return user
}

func HitUser(userid string)*User{
    var user *User
    result := GetCache(userid)
    if result == nil{
        user = NewUser(bson.M{"User_id": strconv.Atoi(userid)})
        if user == nil{
            return user
        }
        SetCache(userid, user, 3600 * 8)
    }else{
        user = result.(User)
    }
    return user
}

func NewLive(spec M) *Live{
    live := &Live{}
    liveCo.Find(spec).One(live)
    if live.Live_id == 0{
        return nil
    }
    return live
}

func HitLive(liveid string)*Live{
    var live *Live
    result := GetCache(liveid)
    if result == nil{
        live = NewLive(bson.M{"Live_id": strconv.Atoi(liveid)})
        if live == nil{
            return live
        }
        SetCache(liveid, live, 60)
    }else{
        live = result.(Live)
    }
    return live
}

func InitCfgList(spec M, sortBy []string, limit int){
    if(limit == nil && (sortBy == nil || len(sortBy) == 0)){
        iterCfg := cfgCo.Find(spec).Iter()
    }
    else if limit == nil{
        iterCfg := cfgCo.Find(spec).Sort(Fastjoin(",", sortBy)).Iter()
    }
    else if (sortBy == nil || len(sortBy) == 0)){
        iterCfg := cfgCo.Find(spec).Limit(limit).Iter()
    }
    else{
        iterCfg := cfgCo.Find(spec).Limit(limit).Sort(Fastjoin(",", sortBy)).Iter()    
    }

    cfg := Cfg{}
    for iterCfg.Next(&cfg) {
        if cfg.Country == ""{
            continue
        }
        if cfg.Country == "DEFAULT"{
            defaultCfg = &cfg
            continue
        }
        SetCache(cfg.Country, cfg, 3600 * 12)
    }
}

func NewCfg(spec M)*Cfg{
    cfg := &Cfg{}
    cfgCo.Find(spec).One(cfg)
    if cfg.Country == ""{
        return nil
    }
    return cfg
}

func HitCfg(country string)*Cfg{
    var cfg *Cfg
    result := GetCache(country)
    if result == nil{
        cfg = NewCfg(bson.M{"country": country})
        if cfg == nil{
            cfg = &Cfg{}
            cfg.Country = country
            cfg.Config = defaultCfg.Config
        }
        SetCache(cfg.Country, cfg, 3600 * 12)
    }else{
        cfg = result.(Cfg)
    }
    return cfg
}
