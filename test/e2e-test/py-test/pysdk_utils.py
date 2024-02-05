"""
Define some classes and methods to support python SDK test.
"""
import os
import logging
import string
from hashlib import sha384
from ccnp import Eventlog


LOG = logging.getLogger("Python SDK test")

class BinaryBlob:
    """
    Manage the binary blob.
    """
    def __init__(self, data):
        self._data = data

    @property
    def length(self):
        return len(self._data)

    @property
    def data(self):
        return self._data

    def get_bytes(self, pos, count):
        if count == 0:
            return None
        assert pos + count <= self.length
        return (self.data[pos:pos + count], pos + count)

class RTMR(BinaryBlob):
    """
    Data structure for RTMR registers.
    A RTMR register manages a 48-bytes (384-bits) hash value.
    """
    RTMR_COUNT = 4
    RTMR_LENGTH_BY_BYTES = 48

    def __init__(self, data: bytearray = bytearray(RTMR_LENGTH_BY_BYTES)):
        super().__init__(data)

    def __eq__(self, other):
        bytearray_1, _ = self.get_bytes(0, RTMR.RTMR_LENGTH_BY_BYTES)
        bytearray_2, _ = other.get_bytes(0, RTMR.RTMR_LENGTH_BY_BYTES)

        return bytearray(bytearray_1) == bytearray(bytearray_2)

class CCEventLogActor:
    """Event log actor
    The actor to process event logs and do replay
    """
    RUNTIME_REGISTER = 2

    def __init__(self):
        self._boot_time_event_logs = []
        self._runtime_event_logs = []

    def _fetch_boot_time_event_logs(self):
        self._boot_time_event_logs = Eventlog.get_platform_eventlog()

    def _fetch_runtime_event_logs(self):
        ima_measurement_file = "/run/security/integrity/ima/ascii_runtime_measurements"
        with open(ima_measurement_file, encoding="utf-8") as f:
            for line in f:
                self._runtime_event_logs.append(line)

    @staticmethod
    def _replay_single_boot_time_rtmr(event_logs) -> RTMR:
        """Replay single RTMR for boot time events"""
        rtmr = bytearray(RTMR.RTMR_LENGTH_BY_BYTES)

        for event_log in event_logs:
            digest = list(map(int, event_log.digest.strip('[]').split(' ')))
            # pylint: disable-next=consider-using-f-string
            digest_hex = ''.join('{:02x}'.format(i) for i in digest)
            sha384_algo = sha384()
            sha384_algo.update(bytes.fromhex(rtmr.hex() + digest_hex))
            rtmr = sha384_algo.digest()

        return RTMR(rtmr)

    @staticmethod
    def _replay_runtime_rtmr(event_logs, base: RTMR) -> RTMR:
        """Replay runtime measurements based on the runtime event logs"""
        rtmr = bytearray(RTMR.RTMR_LENGTH_BY_BYTES)

        val = base.data.hex()
        for event_log in event_logs:
            elements = event_log.split(" ")
            extend_val = val + elements[2]
            sha384_algo = sha384()
            sha384_algo.update(bytes.fromhex(extend_val))
            val = sha384_algo.hexdigest()

        rtmr = sha384_algo.digest()
        return RTMR(rtmr)

    def replay(self, index:int) -> RTMR:
        """Replay event logs including boot time event logs and runtime event logs to
        generate RTMR values for verification.
        """
        self._fetch_boot_time_event_logs()
        boot_time_event_logs_by_index = {}
        boot_time_event_logs_by_index[index] = []

        for event_log in self._boot_time_event_logs:
            if event_log.reg_idx==index:
                boot_time_event_logs_by_index[event_log.reg_idx].append(event_log)
        # replay boot time event logs
        rtmr_value = CCEventLogActor._replay_single_boot_time_rtmr(boot_time_event_logs_by_index[index])
        ima_flag= True
        with open("/proc/cmdline", encoding="utf-8") as proc_f:
            cmdline = proc_f.read().splitlines()
            if "ima_hash=sha384" not in cmdline[0].split(" "):
                # pylint: disable-next=line-too-long
                LOG.info("IMA over RTMR not enabled. Verify basic boot measurements.")
                ima_flag = False

        # fetch and replay the runtime event logs
        if CCEventLogActor.RUNTIME_REGISTER == index and ima_flag:
            ima_measurement_file = \
                "/run/security/integrity/ima/ascii_runtime_measurements"
            assert os.path.exists(ima_measurement_file), \
                f"Could not find the IMA measurement file {ima_measurement_file}"
            LOG.info("IMA event logs found in the system.\n")
            self._fetch_runtime_event_logs()
            concat_rtmr_value = CCEventLogActor._replay_runtime_rtmr(
                self._runtime_event_logs, rtmr_value)
            rtmr_value = concat_rtmr_value
        return rtmr_value
