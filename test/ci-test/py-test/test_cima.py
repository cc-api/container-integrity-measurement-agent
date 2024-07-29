"""
CIMA test:
1. Verify Event logs with RTMR values
2. Verify CC report can be returned successfully
3. Verify IMR[0], IMR[1] and IMR[3] (container event log hash) is not empty
"""

import logging

from cima import CimaSdk

LOG = logging.getLogger(__name__)

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[
        logging.StreamHandler()
    ]
)

class TestCIMA:
    '''
    Tests for CIMA python SDK
    '''
    def test_eventlog_verify(self):
        '''
        Replay and verify event logs
        '''
        evt = CimaSdk.inst().get_cc_eventlog()
        replay = CimaSdk.inst().replay_cc_eventlog(evt)
        for r in replay:
            LOG.info("Replay IMR[%d]: %s", r, replay[r][12].hex())
            m = CimaSdk.inst().get_cc_measurement([r, 12])
            LOG.info("Read IMR[%d]: %s", r, m.hash.hex())
            assert m.hash == replay[r][12], "Replay IMR value does not match real IMR."

    def test_cc_report(self):
        '''
        Test CC report
        '''
        assert CimaSdk.inst().get_cc_report().dump() != ""

    def test_container_imr(self):
        '''
        Test container IMR
        '''
        for i in [0, 1, 3]:
            m = CimaSdk.inst().get_cc_measurement([i, 12])
            assert m.hash.hex() != "", "IMR value should not empty."
