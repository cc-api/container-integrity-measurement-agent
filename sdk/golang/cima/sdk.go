/*
* Copyright (c) 2024, Intel Corporation. All rights reserved.<BR>
* SPDX-License-Identifier: Apache-2.0
 */

package cima

import (
	"errors"
	"log"

	"github.com/cc-api/evidence-api/common/golang/evidence_api"
	"github.com/cc-api/evidence-api/common/golang/evidence_api/tdx"
)

var _ evidence_api.EvidenceAPI = (*SDK)(nil)

type SDK struct {
}

// GetCCReport implements EvidenceAPI
func (s *SDK) GetCCReport(nonce string, userData string, _ any) (evidence_api.Report, error) {
	client, err := NewClient()
	if err != nil {
		log.Fatalf("[GetCCReport] failed to connect to client with error %v", err)
		return nil, err
	}

	result, err := client.GetCCReportFromServer(userData, nonce)
	if err != nil {
		return nil, err
	}

	switch evidence_api.CC_Type(result.CcType) {
	case evidence_api.TYPE_CC_TDX:
		report, err := tdx.NewTdxReportFromBytes(result.CcReport)
		if err != nil {
			return nil, err
		}
		return report, nil
	default:
	}
	return nil, errors.New("[GetCCReport] get CC report failed")
}

// DumpCCReport implements evidence_api.EvidenceAPI.
func (s *SDK) DumpCCReport(reportBytes []byte) error {
	return nil
}

// GetCCMeasurement implements evidence_api.EvidenceAPI.
func (s *SDK) GetCCMeasurement(index int, alg evidence_api.TCG_ALG) (evidence_api.TcgDigest, error) {
	client, err := NewClient()
	if err != nil {
		log.Fatalf("[GetCCMeasurement] failed to connect to client with error %v", err)
		return evidence_api.TcgDigest{}, err
	}

	result, err := client.GetCCMeasurementFromServer(index, alg)
	if err != nil {
		return evidence_api.TcgDigest{}, err
	}
	return evidence_api.TcgDigest{AlgID: evidence_api.TCG_ALG(result.Measurement.AlgoId), Hash: result.Measurement.Hash}, nil
}

// GetMeasurementCount implements evidence_api.EvidenceAPI.
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

// ReplayCCEventLog implements evidence_api.EvidenceAPI.
func (s *SDK) ReplayCCEventLog(formatedEventLogs []evidence_api.FormatedTcgEvent) map[int]map[evidence_api.TCG_ALG][]byte {
	return evidence_api.ReplayFormatedEventLog(formatedEventLogs)
}

// GetDefaultAlgorithm implements evidence_api.EvidenceAPI.
func (s *SDK) GetDefaultAlgorithm() (evidence_api.TCG_ALG, error) {
	client, err := NewClient()
	if err != nil {
		log.Fatalf("[GetDefaultAlgorithm] failed to connect to client with error %v", err)
		return evidence_api.TPM_ALG_ERROR, err
	}

	result, err := client.GetDefaultAlgorithmFromServer()
	if err != nil {
		return evidence_api.TPM_ALG_ERROR, err
	}
	return evidence_api.TCG_ALG(result.AlgoId), nil
}

// GetCCEventlog implements EvidenceAPI.
func (s *SDK) GetCCEventLog(params ...int32) ([]evidence_api.FormatedTcgEvent, error) {
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

	formatted_log_list := make([]evidence_api.FormatedTcgEvent, len(result))
	for idx, log := range result {
		digests := make([]evidence_api.TcgDigest, len(log.Digests))
		for idx, digest := range log.Digests {
			formattedData := evidence_api.TcgDigest{AlgID: evidence_api.TCG_ALG(digest.AlgoId), Hash: digest.Hash}
			digests[idx] = formattedData
		}
		logParser := evidence_api.TcgEventLogParser{RecNum: int(log.RecNum), ImrIndex: int(log.ImrIndex), EventType: evidence_api.TcgEventType(log.EventType), Digests: digests, EventSize: int(log.EventSize), Event: log.Event, ExtraInfo: log.ExtraInfo}
		if evidence_api.TcgEventType(log.EventType) != evidence_api.IMA_MEASUREMENT_EVENT {
			formattedLog := logParser.Format(evidence_api.TCG_PCCLIENT_FORMAT)
			formatted_log_list[idx] = formattedLog
		} else {
			formattedLog := logParser.Format(evidence_api.TCG_PCCLIENT_FORMAT)
			formatted_log_list[idx] = formattedLog
		}
	}

	return formatted_log_list, nil
}
