"""
RTMR test:
1. Fetch boot time event logs using CCNP sdk and fetch runtime event logs(from IMA) in kernel memory
2. Re-calcuate the overall digest
3. Fetch measurements using CCNP sdk
4. Compare the recalculated values with the rtmrs in the measurements
"""

import pysdk_utils
import logging
import base64
from ccnp import Measurement
from ccnp import MeasurementType

class TestRtmr:

    def test_rtmr0(self):
        cc_event_log_actor = pysdk_utils.CCEventLogActor()
        replay_val0 = cc_event_log_actor.replay(index=0)
        val0 = Measurement.get_platform_measurement(
            MeasurementType.TYPE_TDX_RTMR, None, 0)
        rtmr_val0 = pysdk_utils.RTMR(bytearray(base64.b64decode(val0)))
        assert rtmr_val0 == replay_val0


    def test_rtmr1(self):
        cc_event_log_actor = pysdk_utils.CCEventLogActor()
        replay_val1 = cc_event_log_actor.replay(index=1)
        val1 = Measurement.get_platform_measurement(
            MeasurementType.TYPE_TDX_RTMR, None, 1)
        rtmr_val1 = pysdk_utils.RTMR(bytearray(base64.b64decode(val1)))
        assert rtmr_val1 == replay_val1

    def test_rtmr2(self):
        cc_event_log_actor = pysdk_utils.CCEventLogActor()
        replay_val2 = cc_event_log_actor.replay(index=2)
        val2 = Measurement.get_platform_measurement(
            MeasurementType.TYPE_TDX_RTMR, None, 2)
        rtmr_val2 = pysdk_utils.RTMR(bytearray(base64.b64decode(val2)))
        assert rtmr_val2 == replay_val2

    def test_rtmr3(self):
        cc_event_log_actor = pysdk_utils.CCEventLogActor()
        replay_val3 = cc_event_log_actor.replay(index=3)
        val3 = Measurement.get_platform_measurement(
            MeasurementType.TYPE_TDX_RTMR, None, 3)
        rtmr_val3 = pysdk_utils.RTMR(bytearray(base64.b64decode(val3)))
        assert rtmr_val3 == replay_val3

