package compute

func Handle(room_id string, timestamp float64, country string, related_country []string){
    online_men = OnlineCount()
    user_like = LikeCount()

    live = HitLive(room_id, related_country)
    if live == nil:
        return nil

    cfg = HitCfg(live.Country)

    LIMIT_FACTOR = cfg["config"]["limit_factor"]
    VERIFIED_FACTOR = cfg["config"]["verified_factor"]
    RECOMMEND_FACTOR = cfg["config"]["recommend_factor"]
    ONLINEMEN_FACTOR = cfg["config"]["onlinemen_factor"]
    USER_LIKE_FACTOR = cfg["config"]["user_like_factor"]
    TIME_FACTOR = cfg["config"]["time_factor"]
    UNVERIFIED = cfg["config"]["unverified"]

    user = HitUser(live.User_id)
    if user == nil{
        return nil
    }

    if user.Verified{
        verified_val = VERIFIED_FACTOR    
    }else{
        verified_val = 0
    }

    if UNVERIFIED && verified_val == 0{
        return nil
    }

    if recommend_val, ok := RECOMMEND_FACTOR[str(user.recommend_level)]; !ok{
        recommend_val = 0
    }

    now := time.Now().Unix()
    elapse := (now - (live.T_publish | live.Created_at)/1000) / 60
    for one := range(TIME_FACTOR):
        if elapse >= float(one["min"]) && elapse < float(one["max"]){
            extra = (online_men * ONLINEMEN_FACTOR + user_like * USER_LIKE_FACTOR) * (one["base"] + one["increase"] * elapse)
            break
        }
    else{
        extra = 0
    }

    weight := verified_val + recommend_val + extra

    if weight < LIMIT_FACTOR{
        return nil
    }

    if _, ok = live.country[related_country]; ok && !live.country == country{
        return {"1":{"country":live.Country, "ele":(weight, live.Live_id)}}
    }
    else if user.recommend_wight > 0{
        return {"2":{"country":live.Country, "ele":(user.Recommend_wight, live.Live_id)}}
    }
    else{
        return {"3":{"country":live.Country, "ele":(weight, live.live_id)}}
    }
}

