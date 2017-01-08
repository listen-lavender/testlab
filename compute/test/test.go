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

var cpuprofile *string
var memprofile *string
var memprofilerate *int
var blockprofile *string
var blockprofilerate *int

func startCPUProfile() {
    if *cpuprofile != "" {
        f, err := os.Create(*cpuprofile)
        if err != nil {
            log.Fatal(err)
            return
        }
        if err := pprof.StartCPUProfile(f); err != nil {
            log.Fatal(err)
            f.Close()
            return
        }
    }
}

func stopCPUProfile() {
    if *cpuprofile != "" {
        pprof.StopCPUProfile() // 把记录的概要信息写到已指定的文件
    }
}

func startMemProfile() {
    if *memprofile != "" && *memprofilerate > 0 {
        runtime.MemProfileRate = *memprofilerate
    }
}

func stopMemProfile() {
    if *memprofile != "" {
        f, err := os.Create(*memprofile)
        if err != nil {
            log.Fatal(err)
            return
        }
        if err = pprof.WriteHeapProfile(f); err != nil {
            log.Fatal(err)
        }
        f.Close()
    }
}

func startBlockProfile() {
    if *blockprofile != "" && *blockprofilerate > 0 {
        runtime.SetBlockProfileRate(*blockprofilerate)
    }
}

func stopBlockProfile() {
    if *blockprofile != "" && *blockprofilerate >= 0 {
        f, err := os.Create(*blockprofile)
        if err != nil {
            log.Fatal(err)
            return
        }
        if err = pprof.Lookup("block").WriteTo(f, 0); err != nil {
            log.Fatal(err)
        }
        f.Close()
    }
}

func main() {
    cpuprofile = flag.String("cpuprofile", "", "write cpu profile to file")
    memprofile = flag.String("memprofile", "", "write mem profile to file")
    memprofilerate = flag.Int("memprofilerate", 0, "rate of write mem profile to file")
    blockprofile = flag.String("blockprofile", "", "write block profile to file")
    blockprofilerate = flag.Int("blockprofilerate", 0, "rate of write block profile to file")
    flag.Parse()
    startCPUProfile()
    startMemProfile()
    startBlockProfile()
    
    // if *cpuprofile != "" {
    //     f, err := os.Create(*cpuprofile)
    //     if err != nil {
    //         log.Fatal(err)
    //     }
    //     pprof.StartCPUProfile(f)
    //     defer pprof.StopCPUProfile()
    // }
    defer stopCPUProfile()
    defer stopMemProfile()
    defer stopBlockProfile()

    runtime.GOMAXPROCS(runtime.NumCPU())
    println("cpu", runtime.NumCPU())
	start := time.Now().Unix()
    compute.TraversalCountry(compute.HotRoomSet, []string{"TH","US","ID","SG","NG","PH","ZA","IO","MX"}, compute.HandleHot)
    // compute.TraversalCountry(compute.HotRoomSet, []string{"TH","US","ID","SG","NG","PH","ZA","IO","MX","CO","AR","ES","PE","VE","CL","EC","GT","CU","PT","BO","DO","HN","SV","PY","NI","CR","PR","UY","PA","GN","MY","VN","TW","IN","CN","BR","KH","MM","SY","GB","LA","DE","HK","JP","BH","KR","CH","IQ","CZ","SA","NP","IL","AE","IE","NZ","SD","ZG","FR","AU","BD","QA","RU","OM","SE","GR","GL","IR","AL","KE","GD","GP","IT","CA","AF","CM","SI","DK","UM","PK","NA","MN","AO","NO","LY","EG","EE","KW","MO","AD","VG","FI","BM","AX","KZ","AT","NL","TJ","YE","BT"}, compute.HandleHot)
	end := time.Now().Unix()
	println("total: ", end-start, "seconds")
}
