package compute

import (
	"gopkg.in/redis.v5"
	"strconv"
)

func OnlineCount(roomid string) int64 {
	if DEBUG{
		defer trace("OnlineCount")()
	}
	count, _ := client.ZCard(Fastjoin(":", "room", roomid, "online")).Result()
	countRobot, _ := client.SCard(Fastjoin(":", "room", roomid, "robot")).Result()
	return (count - countRobot)
}

func LikeCount(roomid string) float64 {
	if DEBUG{
		defer trace("LikeCount")()
	}
	score, _ := client.ZScore("room:like", roomid).Result()
	return score
}

func GiftCount(roomid string) int {
	if DEBUG{
		defer trace("GiftCount")()
	}
	total_str, _ := client.HGet("room:gift", roomid).Result()
	total, _ := strconv.Atoi(total_str)
	return total
}

func LiveFactor(country string) (float64, float64, map[string]float64, bool, float64, float64, float64) {
	if DEBUG{
		defer trace("LikeFactor")()
	}
	cfg := HitCfg(country)
	limitFactor := cfg.Config.Limit_factor
	verifiedFactor := cfg.Config.Verified_factor
	recommendFactor := cfg.Config.Recommend_factor
	unverified := cfg.Config.Unverified
	onlineFactor := cfg.Config.Onlinemen_factor
	likeFactor := cfg.Config.User_like_factor
	giftFactor := cfg.Config.Gift_factor
	return limitFactor, verifiedFactor, recommendFactor, unverified, onlineFactor, likeFactor, giftFactor
}

func ComputeExtra(roomid string, onlineFactor float64, likeFactor float64, giftFactor float64) float64 {
	if DEBUG{
		defer trace("ComputeExtra")()
	}
	onlineMen := float64(OnlineCount(roomid))
	userLike := LikeCount(roomid)
	giftCoin := float64(GiftCount(roomid))
	extra := onlineMen*onlineFactor + userLike*likeFactor + giftCoin*giftFactor
	return extra
}

func SetCache(key string, val interface{}, expire int) {
	if DEBUG{
		defer trace("SetCache")()
	}
	cachePool.Set([]byte(key), JsonEncode(val), expire)
}

func GetCache(key string) interface{} {
	if DEBUG{
		defer trace("GetCache")()
	}
	val, err := cachePool.Get([]byte(key))
	if err != nil {
		return nil
	} else {
		return JsonDecode(val)
	}
}

func RoomidList() []string {
	if DEBUG{
		defer trace("RoomidList")()
	}
	roomidList, _ := client.SMembers("room:active").Result()
	return roomidList
}

func UpdateZset(key string, val string, weight float64) {
	if DEBUG{
		defer trace("UpdateZset")()
	}
	z := redis.Z{
		Score:  weight,
		Member: val,
	}
	client.ZAdd(key, z)
}

func UpdateZsetRef(key string, val string, weight float64) {
	if DEBUG{
		defer trace("UpdateZsetRef")()
	}
	z := redis.Z{
		Score:  weight,
		Member: val,
	}
	client.ZAdd(key, z)
}

func ClearLiveid(liveZset string, liveZsetRef string) {
	if DEBUG{
		defer trace("ClearLiveid")()
	}
	var zRange redis.ZRangeBy
	var count int64
	var liveidList []string

	count, _ = client.ZCount(liveZsetRef, "0", "0").Result()
	zRange = redis.ZRangeBy{
		Min:    "0",
		Max:    "0",
		Offset: 0,
		Count:  count,
	}

	liveidList, _ = client.ZRangeByScore(liveZsetRef, zRange).Result()

	for _, liveid := range liveidList {
		client.ZRem(liveZset, liveid)
	}
	client.ZRemRangeByScore(liveZsetRef, "0", "0")

	count, _ = client.ZCount(liveZsetRef, "1", "1").Result()
	zRange = redis.ZRangeBy{
		Min:    "1",
		Max:    "1",
		Offset: 0,
		Count:  count,
	}

	liveidList, _ = client.ZRangeByScore(liveZsetRef, zRange).Result()

	for _, liveid := range liveidList {
		UpdateZset(liveZset, liveid, 0)
		// z := redis.Z{
		//     Score: 0,
		//     Member: liveid,
		// }
		// client.ZAdd(liveZsetRef, z)
	}
}
