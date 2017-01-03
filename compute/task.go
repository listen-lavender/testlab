package compute

import (
	"sort"
	"time"
	"sync"
)

var wg sync.WaitGroup

func TraversalCountry(prefix string, curr_countries []string, handle func(string, int64, string, []string, chan *Packet)interface{}) {
	if DEBUG{
		defer trace("TraversalCountry")()
	}
	acc := 0
	stop := false
	feedback := make(chan int)
	length := len(curr_countries)
	for _, country := range curr_countries {
		liveZset := Fastjoin("", prefix, country)
		liveZsetRef := Fastjoin("", prefix, country, ":ref")
		// wg.Add(1)
		// go GenerateCache(country, liveZset, liveZsetRef, handle)
		go GenerateCache(country, liveZset, liveZsetRef, feedback, handle)
	}
	// wg.Wait()
	for {
		select {
		case <-feedback:
			println(acc, length, "=====1")
			acc = acc + 1
			if acc >= length{
				stop = true
				break
			}
		}
		if stop{
			break
		}
	}
}

func GenerateCache(country string, liveZset string, liveZsetRef string, feedback chan int, handle func(string, int64, string, []string, chan *Packet) interface{}) {
// func GenerateCache(country string, liveZset string, liveZsetRef string, handle func(string, int64, string, []string, chan *Packet) interface{}) {
	if DEBUG{
		defer trace("GenerateCache")()
	}
	timestamp := time.Now().Unix() * 1000

	var length int
	acc :=0
	stop := false
	cfg := HitCfg(country)

	refCountry := []string{country}
	refCountry = append(refCountry, cfg.Config.Country...)

	bufferWeight := make(map[string][]float64)
	bufferLiveid := make(map[string][]string)
	buoys := make(map[string]float64)

	roomidList := RoomidList()
	collect := make(chan *Packet)
	length = len(roomidList)
	for _, roomid := range roomidList {
		go handle(roomid, timestamp, country, refCountry, collect)
	}

	for {
		select {
		case packet := <-collect:
			acc = acc + 1
			println(acc, length, country, "=====2")
			if acc >= length{
				stop = true
			}
			if packet == nil {
				if stop{
					break
				}else{
					continue
				}
			}
			if _, ok := buoys[packet.Index]; ok {
                bufferWeight[packet.Index] = append(bufferWeight[packet.Index], packet.Weight)
                bufferLiveid[packet.Index] = append(bufferLiveid[packet.Index], packet.Liveid)
                buoys[packet.Index] = Max(buoys[packet.Index], packet.Weight)
            } else {
                bufferWeight[packet.Index] = []float64{packet.Weight}
                bufferLiveid[packet.Index] = []string{packet.Liveid}
                buoys[packet.Index] = packet.Weight
            }
		}
		if stop{
			break
		}
	}

	inds := MapKeys(buoys)
	length = len(inds)

	sort.Sort(sort.Reverse(sort.StringSlice(inds)))

	for ind, key := range inds {
		if ind > 0 {
			buoys[key] = buoys[key] + buoys[inds[ind-1]]
		}
	}

	sort.Strings(inds)

	var base float64
	for indOut, key := range inds {
		if (indOut + 1) < length {
			base = buoys[key]
		} else {
			base = 0.0
		}
		for indIn, weight := range bufferWeight[key] {
			UpdateZsetMock(liveZset, bufferLiveid[key][indIn], base + weight)
			UpdateZsetRefMock(liveZsetRef, bufferLiveid[key][indIn], 1)
		}
	}
	ClearLiveidMock(liveZset, liveZsetRef)
	// wg.Done()
	feedback <- 1
}
