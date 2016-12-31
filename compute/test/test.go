package main

import (
	"../../compute"
)

func main() {
	compute.TraversalCountry(compute.HotRoomSet, []string{"ID"}, compute.HandleHot)
	// compute.TraversalCountry(compute.HotRoomSet, []string{"TH", "ID", "CN"}, compute.HandleHot)
}
