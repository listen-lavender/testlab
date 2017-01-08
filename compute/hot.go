package compute

import (
	"strconv"
)

func HandleHot(roomid string, timestamp int64, country string, refCountry []string, collect chan *Packet) interface{}{
	if DEBUG{
		defer trace("HandleHot")()
	}
	lsession := gsession.Clone()
	defer lsession.Close()
	
	db := lsession.DB("kittylive")
	live, limitFactor, verifiedFactor, recommendFactor, unverified := HitLive(roomid, refCountry, db)
	if live.Live_id > 0 {
		collect <- nil
		return nil
	}

	userid := strconv.Itoa(live.User_id)
	user := HitUser(userid, db)
	if user.User_id > 0 {
		collect <- nil
		return nil
	}

	var verifiedVal float64

	if user.Verified {
		verifiedVal = verifiedFactor
	} else {
		verifiedVal = 0
	}

	if unverified && verifiedVal == 0.0 {
		collect <- nil
		return nil
	}

	var recommendVal float64
	var ok bool

	if recommendVal, ok = recommendFactor[HitUserIdentity(userid, db)]; !ok {
		recommendVal = 0.0
	}

	weight := verifiedVal + recommendVal + live.Extra

	if weight < limitFactor {
		collect <- nil
		return nil
	}

	recommendWeight := HitUserTop(userid, db)

	var packet Packet
	if !(live.Country == country) {
		packet = Packet{
			Index:"3",
			Country:live.Country,
			Weight:weight,
			Liveid:strconv.Itoa(live.Live_id),
		}
	} else if recommendWeight > 0.0 {
		packet = Packet{
			Index:"1",
			Country:live.Country,
			Weight:recommendWeight,
			Liveid:strconv.Itoa(live.Live_id),
		}
	} else {
		packet = Packet{
			Index:"2",
			Country:live.Country,
			Weight:weight,
			Liveid:strconv.Itoa(live.Live_id),
		}
	}
	collect <- &packet
	return nil
}
