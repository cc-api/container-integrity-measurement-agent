// Define some structures and functions to support golang SDK test.

package gosdk_test

import (
            "ccnp/eventlog"
            "log"
            "strings"
            "crypto/sha512"
            "encoding/hex"
            "io/ioutil"
            "strconv"
    )


const rtmrCount int = 4
const rtmrLength int = 48
const runtimeRegister uint32 = 2
const imaFile string  = "/run/security/integrity/ima/ascii_runtime_measurements"

type rtmr struct{
    data    []byte
}


type eventLogActor struct{
    bootTimeEventlogs       []eventlog.CCEventLogEntry
    runTimeEventlogs        []string
}


func newEventLogActor() *eventLogActor {
    bootTimeEventlogs,err := eventlog.GetPlatformEventlog()
    if err != nil {
            log.Fatalf("Get eventlog error: %v", err)
    }

    imaFlag := true
    f, err := ioutil.ReadFile("/proc/cmdline")
    if err != nil {
            log.Fatalf("Get cmdline error: %v", err)
    }
    cmdline := string(f)
    if !strings.Contains(cmdline, "ima_hash=sha384"){
        imaFlag = false
    }

    var runTimeEventlogs []string
    if imaFlag == true{
        f, err := ioutil.ReadFile(imaFile)
        if err != nil {
            log.Fatalf("Read file: %v error: %v", imaFile, err)
        }
        imaStr := string(f)
        for _, runTimeEventlog := range strings.Split(imaStr, "\n") {
            runTimeEventlogs = append(runTimeEventlogs, runTimeEventlog)
        }
    }

    return &eventLogActor{bootTimeEventlogs: bootTimeEventlogs,runTimeEventlogs: runTimeEventlogs}
}


func (e eventLogActor) replayBootTime(index uint32) *rtmr {
        rtmrVal := make([]byte,rtmrLength)
       

        for _,bootTimeEventlog := range e.bootTimeEventlogs{
            if bootTimeEventlog.RegIdx == index {
                var digestHex string
                digest := string(bootTimeEventlog.Digest)
                for _, val := range strings.Split(strings.Trim(digest, "[]"), " ") {
                    valInt,_ := strconv.Atoi(val)
                    valHex := strconv.FormatInt(int64(valInt), 16)
                     if len(valHex) == 1 {
                       valHex = "0" + valHex
                   }
                   digestHex =  digestHex+valHex
                }

                h := sha512.New384()
                rtmrValHex := hex.EncodeToString(rtmrVal)
                combVal,_ := hex.DecodeString(rtmrValHex+digestHex)
                h.Write(combVal)
                rtmrVal = h.Sum(nil)
            }}
      
        return &rtmr{data:rtmrVal}
}

func (e eventLogActor) replayRunTime(baseRtmr *rtmr) *rtmr {
        extendVal := baseRtmr.data
        for _,runTimeEventlog := range e.runTimeEventlogs{
            extendValHex := hex.EncodeToString(extendVal)
            elements := strings.Fields(runTimeEventlog)
            if len(elements) != 0{
                h := sha512.New384()
                combVal,_:=hex.DecodeString(extendValHex + elements[1])
                h.Write(combVal)
                extendVal = h.Sum(nil)
            }
        }

        return &rtmr{data:extendVal}
}



func (e eventLogActor) replay(index uint32) []byte {
    rtmrValue := e.replayBootTime(index)
    if index == runtimeRegister && len(e.runTimeEventlogs) != 0{
        rtmrValue = e.replayRunTime(rtmrValue)
    }
    return rtmrValue.data
}


