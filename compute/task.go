package compute

import (
	"fmt"
	"sort"
	"time"
)

func TraversalCountry(prefix string, curr_countries []string, handle func(string, int64, string, []string) (string, string, float64, string)) {
	for _, country := range curr_countries {
		liveZset := Fastjoin("", prefix, country)
		liveZsetRef := Fastjoin("", prefix, country, ":ref")
		// go GenerateCache(country, liveZset, liveZsetRef, handle)
		GenerateCache(country, liveZset, liveZsetRef, handle)
	}
}

func GenerateCache(country string, liveZset string, liveZsetRef string, handle func(string, int64, string, []string) (string, string, float64, string)) {
	timestamp := time.Now().Unix() * 1000

	cfg := HitCfg(country)

	refCountry := []string{country}
	refCountry = append(refCountry, cfg.Config.Country...)

	bufferWeight := make(map[string][]float64)
	bufferLiveid := make(map[string][]string)
	buoys := make(map[string]float64)

	for _, roomid := range RoomidList() {
		index, _, weight, liveid := handle(roomid, timestamp, country, refCountry)
		if index == "" {
			continue
		}
		if _, ok := buoys[index]; ok {
			bufferWeight[index] = append(bufferWeight[index], weight)
			bufferLiveid[index] = append(bufferLiveid[index], liveid)
			buoys[index] = Max(buoys[index], weight)
		} else {
			bufferWeight[index] = []float64{weight}
			bufferLiveid[index] = []string{liveid}
			buoys[index] = weight
		}
	}

	indexes := MapKeys(buoys)
	length := len(indexes)

	sort.Sort(sort.Reverse(sort.StringSlice(indexes)))

	for index, key := range indexes {
		if index > 0 {
			buoys[key] = buoys[key] + buoys[indexes[index-1]]
		}
	}

	sort.Strings(indexes)

	var base float64
	for indexOut, key := range indexes {
		if (indexOut + 1) < length {
			base = buoys[key]
		} else {
			base = 0.0
		}
		for indexIn, weight := range bufferWeight[key] {
			fmt.Println(liveZset, bufferLiveid[key][indexIn], base+weight)
			fmt.Println(liveZsetRef, bufferLiveid[key][indexIn], 1)
			// UpdateZset(liveZset, bufferLiveid[key][indexIn], base + weight)
			// UpdateZsetRef(liveZsetRef, bufferLiveid[key][indexIn], 1)
		}
	}

	// ClearLiveid(liveZset, liveZsetRef)
}
