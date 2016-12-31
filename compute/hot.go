package compute

import (
	"strconv"
)

func HandleHot(roomid string, timestamp int64, country string, refCountry []string) (string, string, float64, string) {
	live, limitFactor, verifiedFactor, recommendFactor, unverified := HitLive(roomid, refCountry)
	if live == nil {
		return "", "", 0.0, ""
	}

	userid := strconv.Itoa(live.User_id)
	user := HitUser(userid)
	if user == nil {
		return "", "", 0.0, ""
	}

	var verifiedVal float64

	if user.Verified {
		verifiedVal = verifiedFactor
	} else {
		verifiedVal = 0
	}

	if unverified && verifiedVal == 0.0 {
		return "", "", 0.0, ""
	}

	var recommendVal float64
	var ok bool

	if recommendVal, ok = recommendFactor[HitUserIdentity(userid)]; !ok {
		recommendVal = 0.0
	}

	weight := verifiedVal + recommendVal + live.Extra

	if weight < limitFactor {
		return "", "", 0.0, ""
	}

	recommendWeight := HitUserTop(userid)

	if !(live.Country == country) {
		return "3", live.Country, weight, strconv.Itoa(live.Live_id)
	} else if recommendWeight > 0.0 {
		return "1", live.Country, recommendWeight, strconv.Itoa(live.Live_id)
	} else {
		return "2", live.Country, weight, strconv.Itoa(live.Live_id)
	}
}
