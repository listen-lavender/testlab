package compute

import (
	"encoding/json"
	"github.com/coocood/freecache"
	"fmt"
	"strings"
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
	println(string(str))
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

func MapKeys(data map[string]interface{}) []string {
	keys := make([]string, 0, len(data))
	for key, _ := range data {
		keys = append(keys, key)
	}
	return keys
}

func MapVals(data map[string]interface{}) []interface{} {
	vals := make([]interface{}, 0, len(data))
	for _, val := range data {
		vals = append(vals, val)
	}
	return vals
}
