/*
RTMR test:
1. Fetch boot time event logs using CCNP sdk and fetch runtime event logs(from IMA) in kernel memory
2. Re-calcuate the overall digest
3. Fetch measurements using CCNP sdk
4. Compare the recalculated values with the rtmrs in the measurements
*/

package gosdk_test

import (
            "github.com/intel/confidential-cloud-native-primitives/sdk/golang/ccnp/measurement"
             pb "github.com/intel/confidential-cloud-native-primitives/sdk/golang/ccnp/measurement/proto"
            "testing"
            "bytes"
        )

func TestRtmr(t *testing.T) {
        t.Run("rtmr0", func(t *testing.T) {
                eventLogActor := newEventLogActor()
                replayVal0 :=  eventLogActor.replay(0)
                val0,_ :=  measurement.GetPlatformMeasurement(measurement.WithMeasurementType(pb.CATEGORY_TDX_RTMR), measurement.WithRegisterIndex(0))
                if  !bytes.Equal(replayVal0, val0.(measurement.TDXRtmrInfo).TDXRtmrRaw){
                        t.Error("rtmr0 replay:fail")
                }})

        t.Run("rtmr1", func(t *testing.T) {
                eventLogActor := newEventLogActor()
                replayVal1 :=  eventLogActor.replay(1)
                val1,_ :=  measurement.GetPlatformMeasurement(measurement.WithMeasurementType(pb.CATEGORY_TDX_RTMR), measurement.WithRegisterIndex(1))
                if  !bytes.Equal(replayVal1, val1.(measurement.TDXRtmrInfo).TDXRtmrRaw){
                        t.Error("rtmr1 replay:fail")
                }})
        t.Run("rtmr2", func(t *testing.T) {
                eventLogActor := newEventLogActor()
                replayVal2 :=  eventLogActor.replay(2)
                val2,_ :=  measurement.GetPlatformMeasurement(measurement.WithMeasurementType(pb.CATEGORY_TDX_RTMR), measurement.WithRegisterIndex(2))
                if  !bytes.Equal(replayVal2, val2.(measurement.TDXRtmrInfo).TDXRtmrRaw){
                        t.Error("rtmr2 replay:fail")
                }})

        t.Run("rtmr3", func(t *testing.T) {
                eventLogActor := newEventLogActor()
                replayVal3 :=  eventLogActor.replay(3)
                val3,_ :=  measurement.GetPlatformMeasurement(measurement.WithMeasurementType(pb.CATEGORY_TDX_RTMR), measurement.WithRegisterIndex(3))
                if  !bytes.Equal(replayVal3, val3.(measurement.TDXRtmrInfo).TDXRtmrRaw){
                        t.Error("rtmr3 replay:fail")
                }})


        }

