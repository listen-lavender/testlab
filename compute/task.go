package compute

import (
    "fmt"
    "time"
    "sort"
)

func TraversalCountry(prefix, curr_countries, handle func()){
    for _, country := range(curr_countries){
        liveZset := Fastjoin("", prefix, country)
        liveZsetRef := Fastjoin("", prefix, country, ":ref")
        go GenerateCache(country, liveZset, liveZsetRef, handle)
    }
}

func GenerateCache(country, liveZset, liveZsetRef, handle func()){
    timestamp := time.Now().Unix() * 1000
    related := []
    normal := []
    top := []
    maxWeight := 0

    cfg = HitCfg(country)

    related_country = [country, ]
    related_country = extend(related_country, cfg["config"]["country"])

    buffer := make(map[string]string)
    buoys := make(map[string]float64)

    for _, room_id := range(RoomidList()){
        index, data := handle(int(room_id), timestamp, country, related_country)
        if _, ok := buffer[index]; ok{
            buffer[index] = append(buffer[index], data)
            buoys[index] = data["weight"]
        }else{
            buffer[index] = [data, ]
            buoys[index] = Max(buoys[index], data["weight"])
        }
    }

    indexes := MapKeys(buffer)
    length := len(indexes)

    sort.Sort(sort.Reverse(sort.StringSlice(indexes)))

    for index, key :=range(indexes){
        if index > 0{
            buoys[key] = buoys[key] + buoys[indexes[index - 1]]
        }
    }

    sort.Strings(indexes)

    for indexOut, key :=range(indexes){
        if (indexOut + 1) < length{
            base := buoys[indexes[index + 1]]
        }else{
            base := 0
        }
        for indexIn, data := range(buffer[key]){
            UpdateZset(liveZset, data["liveid"], data["weight"] + base)
            UpdateZsetRef(liveZsetRef, data["liveid"], 1)
        }
    }
    
    ClearLiveid(liveZset, liveZsetRef)
}