package main

import (
    "fmt"
    "bytes"
    "gopkg.in/redis.v5"
)

var buffer bytes.Buffer

func Fastjoin(separator string, args ...string) string {
    last := len(args) - 1
    if separator != "" {
        for k := 0; k < last; k++ {
            buffer.WriteString(args[k])
            buffer.WriteString(separator)
        }
    } else {
        for k := 0; k < last; k++ {
            buffer.WriteString(args[k])
        }
    }
    buffer.WriteString(args[last])
    str := buffer.String()
    buffer.Reset()
    return str
}

func main() {

    ACTIVE_ROOM_SET := "room:active"
    HOT_ROOM_SET := "room:hot"

    client := redis.NewClient(&redis.Options{
        Addr:     "172.31.0.74:6379",
        Password: "4229e03326a029b60d839ab0a063fe85d3849f45a9b908cd013f3808d602d06b", // no password set
        DB:       0,  // use default DB
    })

    // pong, err := client.Ping().Result()
    // fmt.Println(pong, err)

    roomidList, _ := client.SMembers(ACTIVE_ROOM_SET).Result()
    for roomid := range(roomidList){
        fmt.Println("roomid: ", roomid)
    }

    count, _ := client.ZCount(Fastjoin("", HOT_ROOM_SET, "TH"), "-inf", "+inf").Result()

    ascRange := redis.ZRangeBy{
        Min: "-inf",
        Max: "+inf",
        Offset: 0,
        Count: count,
    }

    descRange := redis.ZRangeBy{
        Min: "-inf",
        Max: "+inf",
        Offset: 0,
        Count: count,
    }

    liveidList1, _ := client.ZRangeByScore(Fastjoin("", HOT_ROOM_SET, "TH"), ascRange).Result()
    for _, liveid := range(liveidList1){
        fmt.Println("roomid: ", liveid)
    }
    liveidList2, _ := client.ZRangeByScoreWithScores(Fastjoin("", HOT_ROOM_SET, "TH"), ascRange).Result()
    for _, liveid := range(liveidList2){
        fmt.Println("roomid: ", liveid.Score, liveid.Member)
    }
    liveidList3, _ := client.ZRevRangeByScore(Fastjoin("", HOT_ROOM_SET, "TH"), descRange).Result()
    for _, liveid := range(liveidList3){
        fmt.Println("roomid: ", liveid)
    }
    liveidList4, _ := client.ZRevRangeByScoreWithScores(Fastjoin("", HOT_ROOM_SET, "TH"), descRange).Result()
    for _, liveid := range(liveidList4){
        fmt.Println("roomid: ", liveid.Score, liveid.Member)
    }
}
