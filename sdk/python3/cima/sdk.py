# Copyright (c) 2023, Intel Corporation. All rights reserved.<BR>
# SPDX-License-Identifier: Apache-2.0

"""
This package provides the definitions and helper class for CIMA SDK.
"""

import logging
import os
from typing import Optional
import grpc

from evidence_api.api import EvidenceApi
from evidence_api.ccreport import CcReport
from evidence_api.tcg import TcgAlgorithmRegistry
from evidence_api.tcg import TcgDigest
from evidence_api.tcg import TcgImrEvent
from evidence_api.tcg import TcgPcClientImrEvent
from evidence_api.tdx.quote import TdxQuote
# pylint: disable=E1101
from cima import cima_server_pb2
from cima import cima_server_pb2_grpc

LOG = logging.getLogger(__name__)

# Default gRPC timeout
TIMEOUT = 60

class CimaSdk(EvidenceApi):
    """CIMA SDK class

    This class is a client to connect to CIMA Server and do gRPC call getting the
    server.

    Attributes:
        _server (str): The gRPC server to connect.
        _channel (Channel): The gRPC channel, thread-safe.
        _stub (cimaStub): The get CIMA stub for gRPC.
    """
    _inst = None

    @classmethod
    def inst(cls):
        """Singleton instance function."""
        if cls._inst is None:
            cls._inst = cls()
        return cls._inst

    def __init__(self, server: str="unix:/run/cima/uds/cima-server.sock"):
        """Initialize a gRPC client object

        This constructor initializes gRPC client object with Unix Domain Socket (UDS)
        path. And prepare default atrributes.

        Args:
            server (str): gRPC server UDS path, default is /run/cima/uds/cima-server.sock

        Raises:
            ValueError: If server UDS path is not valid.
        """
        if len(server) == 0 or server[:5] != "unix:":
            raise ValueError("Invalid server path, only unix domain socket supported.")
        self._server = server

        if not os.path.exists(self._server.replace('unix:', '')):
            raise RuntimeError("CIMA server does not start.")
        self._channel = grpc.insecure_channel(self._server,
                                                options=[('grpc.default_authority', 'localhost')])
        try:
            grpc.channel_ready_future(self._channel).result(timeout=TIMEOUT)
        except grpc.FutureTimeoutError as err:
            raise ConnectionRefusedError('Connection to CIMA server failed') from err
        self._stub = cima_server_pb2_grpc.cimaStub(self._channel)

    def _get_container_id(self) -> Optional[str]:
        mountinfo = "/proc/self/mountinfo"
        docker_pattern = "/docker/containers/"
        k8s_pattern = "/kubelet/pods/"
        with open(mountinfo, "r", encoding="utf-8") as f:
            line = f.readline().strip()
            while line:
                if docker_pattern in line:
                    # /var/lib/docker/containers/{container-id}/{file}
                    container_id = line.split(docker_pattern)[-1]
                    container_id = container_id.split('/') [0]
                    return container_id
                if k8s_pattern in line:
                    #  /var/lib/kubelet/pods/{container-id}/{file}
                    container_id = line.split(k8s_pattern)[-1]
                    container_id = container_id.split('/') [0].replace('-', '_')
                    return container_id
                line = f.readline().strip()
        return None

    def get_default_algorithms(self) -> TcgAlgorithmRegistry:
        """Get the default Digest algorithms supported by trusted foundation.

        Different trusted foundation may support different algorithms, for example
        the Intel TDX use SHA384, TPM uses SHA256.

        Beyond the default digest algorithm, some trusted foundation like TPM
        may support multiple algorithms.

        Returns:
            The default algorithms.
        """
        req = cima_server_pb2.GetDefaultAlgorithmRequest()
        resp = self._stub.GetDefaultAlgorithm(req)
        return TcgAlgorithmRegistry(resp.algo_id)

    def get_measurement_count(self) -> int:
        """Get the count of measurement register.

        Different trusted foundation may provide different count of measurement
        register. For example, Intel TDX TDREPORT provides the 4 measurement
        register by default. TPM provides 24 measurement (0~16 for SRTM and 17~24
        for DRTM).

        Beyond the real mesurement register, some SDK may extend virtual measurement
        reigster for additional trust chain like container, namespace, cluster in
        cloud native paradiagm.

        Returns:
            The count of measurement registers
        """
        req = cima_server_pb2.GetMeasurementCountRequest()
        resp = self._stub.GetMeasurementCount(req)
        return resp.count

    def get_cc_measurement(self, imr_select:[int, int]) -> TcgDigest:
        """Get measurement register according to given selected index and algorithms

        Each trusted foundation in CC environment provides the multiple measurement
        registers, the count is update to ``get_measurement_count()``. And for each
        measurement register, it may provides multiple digest for different algorithms.

        Args:
            imr_select ([int, int]): The first is index of measurement register,
                the second is the alrogithms ID

        Returns:
            The object of TcgIMR
        """
        container_id = self._get_container_id()
        if container_id is None:
            LOG.error("Cannot get the container ID, please check the runing environment.")
            return None

        req = cima_server_pb2.GetCcMeasurementRequest(
            container_id=container_id,
            index=imr_select[0], algo_id=imr_select[1]
        )
        resp = self._stub.GetCcMeasurement(req)
        if resp is None or resp.measurement is None:
            LOG.error("CIMA service response is not correct.")
            return None

        return TcgDigest(resp.measurement.algo_id, resp.measurement.hash)

    def get_cc_report(
        self,
        nonce: bytearray = None,
        data: bytearray = None,
        extraArgs = None
    ) -> CcReport:
        """Get the CcReport (i.e. quote) for given nonce and data.

        The CcReport is signing of attestation data (IMR values or hashes of IMR
        values), made by a trusted foundation (TPM) using a key trusted by the
        verifier.

        Different trusted foundation may use different quote format.

        Args:
            nonce (bytearray): against replay attacks.
            data (bytearray): user data
            extraArgs: for TPM, it will be given list of IMR/PCRs

        Returns:
            The ``CcReport`` object. Return None if it fails.
        """
        container_id = self._get_container_id()
        if container_id is None:
            LOG.error("Cannot get the container ID, please check the runing environment.")
            return None

        req = cima_server_pb2.GetCcReportRequest(
            container_id=container_id,
            nonce=nonce, user_data=data
        )
        resp = self._stub.GetCcReport(req)
        if resp is None or resp.cc_type is None or resp.cc_report is None:
            LOG.error("CIMA service response is not correct.")
            return None

        if resp.cc_type == EvidenceApi.TYPE_CC_TDX:
            return TdxQuote(resp.cc_report)

        LOG.error("The SDK does not support %s yet", resp.cc_type)
        return None

    def get_cc_eventlog(self, start:int = None, count:int = None) -> list:
        """Get eventlog for given index and count.

        TCG log in Eventlog. Verify to spoof events in the TCG log, hence defeating
        remotely-attested measured-boot.
        To measure the full CC runtime environment, the eventlog may include addtional
        OS type and cloud native type event beyond the measured-boot.

        Args:
            start(int): the first index of event log to fetch
            count(int): the number of event logs to fetch

        Returns:
            Parsed event logs following TCG Spec.
        """
        container_id = self._get_container_id()
        if container_id is None:
            LOG.error("Cannot get the container ID, please check the runing environment.")
            return None

        req = cima_server_pb2.GetCcEventlogRequest(
            container_id=container_id,
            start=start, count=count
        )
        resp = self._stub.GetCcEventlog(req)
        if resp is None or resp.event_logs is None:
            LOG.error("CIMA service response is not correct.")
            return None

        event_logs = []
        for evt in resp.event_logs:
            if len(evt.digests) > 0:
                digests = []
                for d in evt.digests:
                    digests.append(TcgDigest(d.algo_id, d.hash))
                event_logs.append(TcgImrEvent(evt.imr_index, evt.event_type, digests,
                                              evt.event_size, evt.event))
            else:
                event_logs.append(TcgPcClientImrEvent(evt.imr_index, evt.event_type, evt.digest,
                                                      evt.event_size, evt.event))
        return event_logs
