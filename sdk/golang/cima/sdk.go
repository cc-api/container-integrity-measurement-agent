/*
* Copyright (c) 2024, Intel Corporation. All rights reserved.<BR>
* SPDX-License-Identifier: Apache-2.0
 */

package cima

import (
	"errors"
	"log"

	"github.com/cc-api/cc-trusted-api/common/golang/cctrusted_base"
	"github.com/cc-api/cc-trusted-api/common/golang/cctrusted_base/tdx"
)

var _ cctrusted_base.CCTrustedAPI = (*SDK)(nil)

type SDK struct {
}

// GetCCReport implements CCTrustedAPI
func (s *SDK) GetCCReport(nonce string, userData string, _ any) (cctrusted_base.Report, error) {
	client, err := NewClient()
	if err != nil {
		log.Fatalf("[GetCCReport] failed to connect to client with error %v", err)
		return nil, err
	}

	result, err := client.GetCCReportFromServer(userData, nonce)
	if err != nil {
		return nil, err
	}

	switch cctrusted_base.CC_Type(result.CcType) {
	case cctrusted_base.TYPE_CC_TDX:
		report, err := tdx.NewTdxReportFromBytes(result.CcReport)
		if err != nil {
			return nil, err
		}
		return report, nil
	default:
	}
	return nil, errors.New("[GetCCReport] get CC report failed")
}

// DumpCCReport implements cctrusted_base.CCTrustedAPI.
func (s *SDK) DumpCCReport(reportBytes []byte) error {
	return nil
}

// GetCCMeasurement implements cctrusted_base.CCTrustedAPI.
func (s *SDK) GetCCMeasurement(index int, alg cctrusted_base.TCG_ALG) (cctrusted_base.TcgDigest, error) {
	client, err := NewClient()
	if err != nil {
		log.Fatalf("[GetCCMeasurement] failed to connect to client with error %v", err)
		return cctrusted_base.TcgDigest{}, err
	}

	result, err := client.GetCCMeasurementFromServer(index, alg)
	if err != nil {
		return cctrusted_base.TcgDigest{}, err
	}
	return cctrusted_base.TcgDigest{AlgID: cctrusted_base.TCG_ALG(result.Measurement.AlgoId), Hash: result.Measurement.Hash}, nil
}

// GetMeasurementCount implements cctrusted_base.CCTrustedAPI.
func (s *SDK) GetMeasurementCount() (int, error) {
	client, err := NewClient()
	if err != nil {
		log.Fatalf("[GetMeasurementCount] failed to connect to client with error %v", err)
		return -1, err
	}

	result, err := client.GetMeasurementCountFromServer()
	if err != nil {
		return -1, err
	}
	return int(result.Count), nil
}

// ReplayCCEventLog implements cctrusted_base.CCTrustedAPI.
func (s *SDK) ReplayCCEventLog(formatedEventLogs []cctrusted_base.FormatedTcgEvent) map[int]map[cctrusted_base.TCG_ALG][]byte {
	return cctrusted_base.ReplayFormatedEventLog(formatedEventLogs)
}

// GetDefaultAlgorithm implements cctrusted_base.CCTrustedAPI.
func (s *SDK) GetDefaultAlgorithm() (cctrusted_base.TCG_ALG, error) {
	client, err := NewClient()
	if err != nil {
		log.Fatalf("[GetDefaultAlgorithm] failed to connect to client with error %v", err)
		return cctrusted_base.TPM_ALG_ERROR, err
	}

	result, err := client.GetDefaultAlgorithmFromServer()
	if err != nil {
		return cctrusted_base.TPM_ALG_ERROR, err
	}
	return cctrusted_base.TCG_ALG(result.AlgoId), nil
}

// GetCCEventlog implements CCTrustedAPI.
func (s *SDK) GetCCEventLog(params ...int32) ([]cctrusted_base.FormatedTcgEvent, error) {
	if len(params) > 2 {
		log.Fatalf("Invalid params specified for [GetCCEventlog].")
		return nil, errors.New("Invalid params.")
	}

	client, err := NewClient()
	if err != nil {
		log.Fatalf("[GetCCEventLog] failed to connect to client with error %v", err)
		return nil, err
	}

	result, err := client.GetCCEventLogFromServer(params...)
	if err != nil {
		return nil, err
	}

	formatted_log_list := make([]cctrusted_base.FormatedTcgEvent, len(result))
	for idx, log := range result {
		digests := make([]cctrusted_base.TcgDigest, len(log.Digests))
		for idx, digest := range log.Digests {
			formattedData := cctrusted_base.TcgDigest{AlgID: cctrusted_base.TCG_ALG(digest.AlgoId), Hash: digest.Hash}
			digests[idx] = formattedData
		}
		logParser := cctrusted_base.TcgEventLogParser{RecNum: int(log.RecNum), ImrIndex: int(log.ImrIndex), EventType: cctrusted_base.TcgEventType(log.EventType), Digests: digests, EventSize: int(log.EventSize), Event: log.Event, ExtraInfo: log.ExtraInfo}
		if cctrusted_base.TcgEventType(log.EventType) != cctrusted_base.IMA_MEASUREMENT_EVENT {
			formattedLog := logParser.Format(cctrusted_base.TCG_PCCLIENT_FORMAT)
			formatted_log_list[idx] = formattedLog
		} else {
			formattedLog := logParser.Format(cctrusted_base.TCG_PCCLIENT_FORMAT)
			formatted_log_list[idx] = formattedLog
		}
	}

	return formatted_log_list, nil
}
