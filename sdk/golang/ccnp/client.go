/*
* Copyright (c) 2024, Intel Corporation. All rights reserved.<BR>
* SPDX-License-Identifier: Apache-2.0
 */

package ccnp

import (
	"bufio"
	context "context"
	"log"
	"os"
	"strings"
	"time"

	"github.com/cc-api/cc-trusted-api/common/golang/cctrusted_base"
	pb "github.com/cc-api/confidential-cloud-native-primitives/sdk/golang/ccnp/proto"
	"google.golang.org/grpc"
)

const (
	UDS_PATH = "unix:/run/ccnp/uds/ccnp-server.sock"
)

type Client struct {
	client pb.CcnpClient
}

func NewClient() (Client, error) {
	conn, err := grpc.Dial(UDS_PATH, grpc.WithInsecure())
	if err != nil {
		log.Fatalf("[GetCCReportFromServer] can not connect to CCNP server UDS at %v with error: %v", UDS_PATH, err)
		return Client{}, err
	}

	client := pb.NewCcnpClient(conn)
	return Client{client: client}, nil
}

func GetContainerId() string {
	var mountinfoFile string = "/proc/self/mountinfo"
	var dockerPattern string = "/docker/containers/"
	var k8sPattern string = "/kubelet/pods/"

	file, err := os.Open(mountinfoFile)
	if err != nil {
		log.Fatalf("[GetContainerId] fail to open mountinfo file: %v", err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	for _, line := range lines {
		/*
		 * line format:
		 *      ... /var/lib/docker/containers/{container-id}/{file} ...
		 * sample:
		 */
		if strings.Contains(line, dockerPattern) {
			// /var/lib/docker/containers/{container-id}/{file}
			var res = strings.Split(line, dockerPattern)
			var res1 = res[len(res)-1]
			var ContainerId = strings.Split(res1, "/")[0]

			return ContainerId
		}

		/*
		 * line format:
		 *      ... /var/lib/kubelet/pods/{container-id}/{file} ...
		 * sample:
		 *      2958 2938 253:1 /var/lib/kubelet/pods/a45f46f0-20be-45ab-ace6-b77e8e2f062c/containers/busybox/8f8d892c /dev/termination-log rw,relatime - ext4 /dev/vda1 rw,discard,errors=remount-ro
		 */
		if strings.Contains(line, k8sPattern) {
			// /var/lib/kubelet/pods/{container-id}/{file}
			var res = strings.Split(line, k8sPattern)
			var res1 = res[len(res)-1]
			var res2 = strings.Split(res1, "/")[0]
			var ContainerId = strings.Replace(res2, "-", "_", -1)

			return ContainerId
		}
	}

	log.Fatalf("[GetContainerId] no docker or kubernetes container patter found in /proc/self/mountinfo")
	return ""
}

func (cc *Client) GetCCReportFromServer(userData string, nonce string) (pb.GetCcReportResponse, error) {
	var ContainerId = GetContainerId()
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	response, err := cc.client.GetCcReport(ctx, &pb.GetCcReportRequest{ContainerId: ContainerId, Nonce: &nonce, UserData: &userData})
	if err != nil {
		log.Fatalf("[GetCCReportFromServer] fail to get cc report with error: %v", err)
	}

	return *response, nil
}

func (cc *Client) GetDefaultAlgorithmFromServer() (pb.GetDefaultAlgorithmResponse, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	response, err := cc.client.GetDefaultAlgorithm(ctx, &pb.GetDefaultAlgorithmRequest{})
	if err != nil {
		log.Fatalf("[GetDefaultAlgorithm] fail to get default algorithm with error: %v", err)
		return pb.GetDefaultAlgorithmResponse{}, err
	}

	return *response, nil
}

func (cc *Client) GetMeasurementCountFromServer() (pb.GetMeasurementCountResponse, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	response, err := cc.client.GetMeasurementCount(ctx, &pb.GetMeasurementCountRequest{})
	if err != nil {
		log.Fatalf("[GetMeasurementCount] fail to get measurement count with error: %v", err)
		return pb.GetMeasurementCountResponse{}, err
	}

	return *response, nil
}

func (cc *Client) GetCCMeasurementFromServer(index int, alg cctrusted_base.TCG_ALG) (pb.GetCcMeasurementResponse, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	response, err := cc.client.GetCcMeasurement(ctx, &pb.GetCcMeasurementRequest{ContainerId: GetContainerId(), Index: uint32(index), AlgoId: uint32(alg)})
	if err != nil {
		log.Fatalf("[GetCCMeasurement] fail to get measurement with error: %v", err)
		return pb.GetCcMeasurementResponse{}, err
	}

	return *response, nil
}

func (cc *Client) GetCCEventLogFromServer(params ...int32) ([]*pb.TcgEventlog, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()

	maxSizeOption := grpc.MaxCallRecvMsgSize(32 * 10e6)

	req := pb.GetCcEventlogRequest{ContainerId: GetContainerId()}

	if len(params) != 0 {
		for idx, param := range params {
			// first param represents the start
			if idx == 0 {
				formatted_value := uint32(param)
				req.Start = &formatted_value
			}
			// second param represents the count
			if idx == 1 {
				formatted_value := uint32(param)
				req.Count = &formatted_value
			}
		}
	}

	response, err := cc.client.GetCcEventlog(ctx, &req, maxSizeOption)
	if err != nil {
		log.Fatalf("[GetCCEventlog] fail to get event log with error: %v", err)
		return nil, err
	}

	return response.EventLogs, nil
}
