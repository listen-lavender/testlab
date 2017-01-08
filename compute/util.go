package compute

import (
	"bytes"
	"encoding/json"
	"time"
	"fmt"
)

func JsonDecode(str []byte) map[string]interface{} {
	if DEBUG{
		defer trace("JsonDecode")()
	}
	var dict map[string]interface{}
	err := json.Unmarshal(str, &dict)
	if err != nil {
		println(err)
		return nil
	}
	return dict
}

func JsonEncode(dict interface{}) []byte {
	if DEBUG{
		defer trace("JsonEncode")()
	}
	str, err := json.Marshal(dict)
	if err != nil {
		println(err)
		return nil
	}
	return str
}

func Fastjoin(separator string, args ...string) string {
	if DEBUG{
		defer trace("Fastjoin")()
	}
	var buffer bytes.Buffer
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

func MapKeys(data map[string]float64) []string {
	if DEBUG{
		defer trace("MapKeys")()
	}
	keys := make([]string, 0, len(data))
	for key, _ := range data {
		keys = append(keys, key)
	}
	return keys
}

func IndexOf(obj string, list []string) int {
	if DEBUG{
		defer trace("IndexOf")()
	}
	index := -1
	var src string
	for index, src = range list {
		if obj == src {
			return index
		}
	}
	return index
}

func Max(a float64, b float64) float64 {
	if a > b {
		return a
	}
	return b
}

func Min(a int, b int) int {
	if a < b {
		return a
	}
	return b
}

func trace(funcName string) func() {
	start := time.Now()
	return func() {
		fmt.Printf("%s elapse: %s \n", funcName, time.Since(start))
	}
}

func UpdateZsetMock(key string, val string, weight float64){
	if false{
		println(key, val, weight)
	}
}

func UpdateZsetRefMock(key string, val string, weight float64){
	if false{
		println(key, val, weight)
	}	
}

func ClearLiveidMock(liveZset string, liveZsetRef string){
	if false{
		println(liveZset, liveZsetRef)
	}	
}
