package compute

import (
	"encoding/json"
	"fmt"
)

func JsonDecode(str []byte) map[string]interface{} {
	var dict map[string]interface{}
	err := json.Unmarshal(str, &dict)
	if err != nil {
		fmt.Println(err)
		return nil
	}
	return dict
}

func JsonEncode(dict interface{}) []byte {
	str, err := json.Marshal(dict)
	if err != nil {
		fmt.Println(err)
		return nil
	}
	return str
}

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

func MapKeys(data map[string]float64) []string {
	keys := make([]string, 0, len(data))
	for key, _ := range data {
		keys = append(keys, key)
	}
	return keys
}

func IndexOf(obj string, list []string) int {
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
