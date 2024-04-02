"""
CCNP SDK Example
"""

import logging
import argparse

from ccnp import CcnpSdk

LOG = logging.getLogger(__name__)

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[
        logging.StreamHandler()
    ]
)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="The utility to show how to use CCNP SDK")
    parser.add_argument('-r', action='store_true', help='get cc report', dest='report')
    parser.add_argument('-e', action='store_true', help='get cc eventlog', dest='eventlog')
    parser.add_argument('-m', action='store_true', help='get cc measurement', dest='measurement')
    parser.add_argument('-v', action='store_true', help='verify eventlog', dest='verify')
    args = parser.parse_args()

    if args.report:
        CcnpSdk.inst().get_cc_report().dump()
    elif args.eventlog:
        evt = CcnpSdk.inst().get_cc_eventlog()
        for e in evt:
            e.dump()
    elif args.measurement:
        for i in [0, 1, 3]:
            m = CcnpSdk.inst().get_cc_measurement([i, 12])
            LOG.info("IMR index: %d, hash: %s", i, m.hash.hex())
    elif args.verify:
        evt = CcnpSdk.inst().get_cc_eventlog()
        replay = CcnpSdk.inst().replay_cc_eventlog(evt)
        for r in replay:
            LOG.info("Replay IMR[%d]: %s", r, replay[r][12].hex())
            m = CcnpSdk.inst().get_cc_measurement([r, 12])
            LOG.info("Read IMR[%d]: %s", r, m.hash.hex())
            if m.hash != replay[r][12]:
                LOG.error("Replay IMR value does not match real IMR.")
            else:
                LOG.info("Verify event log replay value successfully.")
    else:
        parser.print_help()
