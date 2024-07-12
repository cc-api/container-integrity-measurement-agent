"""
CCNP SDK Perf Test
"""

import logging
import argparse
import time

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

    args = parser.parse_args()

    inst = CcnpSdk.inst()

    if args.report:
        start_time = time.time()
        report = inst.get_cc_report()
        end_time = time.time()
        elapsed_time = end_time - start_time
        LOG.info(f"Quote - Total command executed in {elapsed_time:.6f} seconds")
        report.dump()
    elif args.eventlog:
        start_time = time.time()
        evt = inst.get_cc_eventlog()
        end_time = time.time()
        elapsed_time = end_time - start_time
        LOG.info(f"Eventlog - Total command executed in {elapsed_time:.6f} seconds")
        for e in evt:
            e.dump()
    elif args.measurement:
        start_time = time.time()
        for i in [0, 1, 3]:
            m = inst.get_cc_measurement([i, 12])
            LOG.info("IMR index: %d, hash: %s", i, m.hash.hex())
        end_time = time.time()
        elapsed_time = end_time - start_time
        LOG.info(f"Measurememt - Total command executed in {elapsed_time:.6f} seconds")
    else:
        parser.print_help()
