package main

import (
	"../../compute"
	"time"
    "runtime"
    "runtime/pprof"
    "flag"
    "log"
    "os"
)

func main() {
    var cpuprofile = flag.String("cpuprofile", "", "write cpu profile to file")
    flag.Parse()
    if *cpuprofile != "" {
        f, err := os.Create(*cpuprofile)
        if err != nil {
            log.Fatal(err)
        }
        pprof.StartCPUProfile(f)
        defer pprof.StopCPUProfile()
    }

    runtime.GOMAXPROCS(runtime.NumCPU())
    println("cpu", runtime.NumCPU())
	start := time.Now().Unix()
	// compute.TraversalCountry(compute.HotRoomSet, []string{"TH", "ID", "SG"}, compute.HandleHot)
    compute.TraversalCountry(compute.HotRoomSet, []string{"TH", "ID"}, compute.HandleHot)
    // compute.TraversalCountry(compute.HotRoomSet, []string{"TH", }, compute.HandleHot)
	end := time.Now().Unix()
	println("total: ", end-start, "seconds")
	// compute.TraversalCountry(compute.HotRoomSet, []string{"TH", "ID", "CN"}, compute.HandleHot)
}
