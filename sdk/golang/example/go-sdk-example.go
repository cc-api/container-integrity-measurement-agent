package main

import (
	b64 "encoding/base64"
	"encoding/binary"
	"log"
	"math"
	"math/rand"
	"os"

	"github.com/cc-api/cc-trusted-api/common/golang/cctrusted_base"
	"github.com/cc-api/confidential-cloud-native-primitives/sdk/golang/ccnp"
)

// func to test GetCCReport()
func testGetCCReport(sdk ccnp.SDK, logger *log.Logger) {
	logger.Println("Call [GetCCReport] to fetch attestation report...")

	num := uint64(rand.Int63n(math.MaxInt64))
	b := make([]byte, 8)
	binary.LittleEndian.PutUint64(b, num)
	nonce := b64.StdEncoding.EncodeToString(b)

	rawBytes := []byte("demo user data")
	userData := b64.StdEncoding.EncodeToString(rawBytes)
	report, err := sdk.GetCCReport(nonce, userData, nil)
	if err != nil {
		logger.Println("Error in fetching cc report.")
		os.Exit(-1)
	}

	logger.Println("Dump the attestation report fetched.")
	report.Dump(cctrusted_base.QuoteDumpFormat(cctrusted_base.QuoteDumpFormatRaw))
	logger.Println("----------------------------------------------------------------------------------")
}

// func to test GetCCMeasurement()
func testGetCCMeasurement(sdk ccnp.SDK, logger *log.Logger) {
	logger.Println("Call [GetCCMeasurement] to fetch measurement for specific IMR[0]...")

	imr_index := 0
	alg := cctrusted_base.TPM_ALG_SHA384

	measurement, err := sdk.GetCCMeasurement(imr_index, alg)
	if err != nil {
		logger.Println("Error in fetching cc measurement.")
		os.Exit(-1)
	}

	logger.Println("Dump measurement fetched.")
	logger.Println("AlgID:  ", measurement.AlgID)
	logger.Println("Digest:")
	logger.Printf("    %02X", measurement.Hash)
	logger.Println("----------------------------------------------------------------------------------")
}

// func to test GetCCEventLog()
func testGetCCEventLog(sdk ccnp.SDK, logger *log.Logger) {
	logger.Println("Call [GetCCEventLog] to fetch cc event logs...")
	/*
	   Another example to set start to 0 and count to 10 for event log retrieval
	   start := int32(0)
	   count := int32(10)

	   eventLogs, err := sdk.GetCCEventLog(start, count)
	*/
	eventLogs, err := sdk.GetCCEventLog()
	if err != nil {
		logger.Println("Error in fetching event logs.")
		os.Exit(-1)
	}

	logger.Println("Total ", len(eventLogs), " of event logs fetched.")
	for _, log := range eventLogs {
		log.Dump()
	}
	logger.Println("----------------------------------------------------------------------------------")
}

func main() {
	logger := log.Default()
	sdk := ccnp.SDK{}

	logger.Println("Call [GetDefaultAlgorithm] to fetch default algorithm...")
	defaultAlg, err := sdk.GetDefaultAlgorithm()
	if err != nil {
		logger.Println("Error in fetching default algorithm.")
		os.Exit(-1)
	}
	logger.Println("Default Algorithm:   ", defaultAlg)
	logger.Println("----------------------------------------------------------------------------------")

	logger.Println("Call [GetMeasurementCount] to fetch measurement count...")
	count, err := sdk.GetMeasurementCount()
	if err != nil {
		logger.Println("Error in fetching measurement count.")
		os.Exit(-1)
	}
	logger.Println("Measurement count:   ", count)
	logger.Println("----------------------------------------------------------------------------------")

	// test GetCCReport() API
	testGetCCReport(sdk, logger)

	// test GetCCMeasurement() API
	testGetCCMeasurement(sdk, logger)

	// test GetCCEventLog() API
	testGetCCEventLog(sdk, logger)
}
