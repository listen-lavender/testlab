package compute

func OnlineCount() int{
    return 0
}

func LikeCount() int{
    return 0
}

func SetCache(key string, val interface{}, expire int){
    cachePool.Set([]byte(key), JsonEncode(val), expire)
}

func GetCache(key string) interface{}{
    val, err := cachePool.Get([]byte(key))
    if err != nil {
        return nil
    } else {
        return JsonDecode(val)
    }
}

func RoomidList()[]string{
    roomidList, _ := client.SMembers(activeRoomSet).Result()
    return roomidList
}

func UpdateZset(key string, val string, weight float64){
    z := redis.Z{
        Score: weight,
        Member: val,
    }
    client.Zadd(key, z)
}

func UpdateZsetRef(key string, val string, weight float64){
    z := redis.Z{
        Score: weight,
        Member: val,
    }
    client.Zadd(key, z)
}

func ClearLiveid(string liveZset, string liveZsetRef){
    var zRange *Range
    var count int
    var liveidList []string

    count, _ = client.ZCount(liveZsetRef, "0", "0").Result()
    zRange = redis.ZRangeBy{
        Min: "0",
        Max: "0",
        Offset: 0,
        Count: count,
    }

    liveidList, _ = client.ZRangeByScore(liveZsetRef, zRange).Result()

    for _, liveid := range(liveidList){
        client.Zrem(liveZset, liveid)
    }
    client.zRemRangeByScore(liveZsetRef, "0", "0")

    count, _ = client.ZCount(liveZsetRef, "1", "1").Result()
    zRange = redis.ZRangeBy{
        Min: "1",
        Max: "1",
        Offset: 0,
        Count: count,
    }

    liveidList, _ = client.ZRangeByScore(liveZsetRef, zRange).Result()

    for _, liveid := range(liveidList){
        UpdateZset(liveZset, liveid, 0)
        // z := redis.Z{
        //     Score: 0,
        //     Member: liveid,
        // }
        // client.Zadd(liveZsetRef, z)
    }
}
