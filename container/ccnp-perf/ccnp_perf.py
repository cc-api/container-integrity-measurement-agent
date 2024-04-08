"""
CCNP Performance Test.
"""

import logging
import concurrent

from concurrent.futures import ThreadPoolExecutor
from multiprocessing import Process, Queue
from threading import Event
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

RESPONSE_TEST_REPEAT_TIME = 20
"""The repeat time for response test.
The current design is 20 times.
"""

THROUGHPUT_TEST_TASK_NUM = 85
"""The number of processes for throughput test.
TODO:
    We should be able to use 1025 for test (see our design for details).
    But our current test implementation has some problem to be solved which seems to be
    related to some limit of "_multiprocessing.SemLock".
"""

THROUGHPUT_TEST_TASK_TIME = 20
"""The time used to collect throughput test data for later average calculation.
The current setting is 20 seconds.
TODO:
    We should be able to use more time for this (e.g. 30 seconds as the current design).
    But some case looks hang when we use more time (e.g. 30 seconds). We need resolve this.
"""

class PerfTask:
    """Struct used for performance test task."""
    def __init__(self, proc, queue):
        """Initialize the PerfTask.
        Args:
            proc: an instance of ``Process`` that run the perf test task.
            queue: an instance of ``Qeueue`` that is used to store the test result.
        """
        self.proc = proc
        self.queue = queue

def _timeout_fn(fn, timeout, ev_stop):
    """Invoke function with timeout.

    Args:
        fn: function to be invoked.
        timeout: specified timeout.
        ev_stop: Event object used to stop the thread in thread pool.
    
    Returns:
        The result returned by function "fn".
    
    The function call to "fn" will be stopped when the specified timeout expires. 
    """
    r = None
    with ThreadPoolExecutor(max_workers=1) as executor:
        future = executor.submit(fn)
        try:
            r = future.result(timeout)
        except concurrent.futures.TimeoutError:
            # Timeout expires.
            # Now it's time to stop the thread and get the result.
            ev_stop.set()
            r = future.result()
    return r

def _repeat_svc_call(svc_call, ev_stop):
    """Repeat the service call in infinite loop.

    Args:
        svc_call: function to invoke the service call.
        ev_stop: Event objectg used to control when to exit the infinite loop.

    Returns:
        The total count of the service call having been invoked.
    """
    cnt = 0
    while True:
        svc_call()
        cnt += 1
        if ev_stop.is_set():
            return cnt

def _cnt_operations(svc_call, res_queue, timeout):
    """Count the operations with specified timeout.

    Args:
        svc_call: function to invoke the service call.
        res_queue: an instance of ``Queue`` used to pass the result to the parent process.
        timeout: time used to repeat the service call.
    """
    ev_stop = Event()
    # pylint: disable=unnecessary-lambda-assignment
    operation = lambda: _repeat_svc_call(svc_call, ev_stop)
    cnt = _timeout_fn(operation, timeout, ev_stop)
    res_queue.put(cnt) # Pass the total count to the parent process.

def _timing_response(svc_call, res_queue):
    """Timing the response of specified service call.
    
    Args:
        svc_call: function to invoke the service call.
        res_queue: an instance of ``Queue`` used to pass the result to the parent process.
    
    The timing result uses nanoscecond as the unit. And it's put into the res_queue which can
    be get by the parent process for further handling.
    """
    t_begin = time.time_ns()
    svc_call()
    t_end = time.time_ns()
    t_cost = t_end - t_begin
    res_queue.put(t_cost) # Pass the total count to the parent process.

def _start_proc(_perf_wrapper_fn, svc_call, timeout):
    """Start a new process to run specified function asynchronously.

    Args:
        _perf_wrapper_fn: a wrapper function for service call. e.g. a function to count the ops.
        svc_call: function to invoke the service call.
        timeout: the function will be stopped when this timeout expires.

    Returns:
        An instances for tuple (``Process``, ``Queue``).

    When the specified timeout expires, the function will be stopped. And the forked process will
    be terminated. The result got by the child process will be stored in the ``Queue``.
    """
    res_queue = Queue()
    p = Process(target=lambda: _perf_wrapper_fn(svc_call, res_queue, timeout))
    p.start()
    return p, res_queue

def _start_proc_and_wait(_perf_wrapper_fn, svc_call):
    """Start a new process to run specified function synchronously.

    Args:
        _perf_wrapper_fn: a wrapper function for service call. e.g. a function to do the timing.
        svc_call: function to invoke the service call.

    Returns:
        The result returned by the wrapper function. e.g. the timing result.
    """
    res_queue = Queue()
    p = Process(target=lambda: _perf_wrapper_fn(svc_call, res_queue))
    p.start()
    # NOTE: The order matters here. There could be some deadlock if "join" before
    # "get" according to the document of Python:
    # https://docs.python.org/3/library/multiprocessing.html#programming-guidelines
    r = res_queue.get()
    p.join()
    return r

def _test_throughput(svc_call):
    """Generic throughput test workflow.

    Args:
        svc_call: function to invoke the service call.
    """
    # 1. Start N processes to simulate N apps (see our design for details).
    task_num = THROUGHPUT_TEST_TASK_NUM
    time_total = THROUGHPUT_TEST_TASK_TIME
    tasks = []
    for _ in range(task_num):
        # 2. Each process invokes the CCNP API (either via SDK or service directly) repeatedly
        # until the timeout T expires (see our design details).
        p, res_queue = _start_proc(_cnt_operations, svc_call, time_total)
        tasks.append(PerfTask(p, res_queue))

    # 3. Calculate the average ops (total count of completed API calls in N threads divided by T).
    cnt_total = 0
    for t in tasks:
        cnt = t.queue.get()
        t.proc.join()
        cnt_total += cnt
    ops_avg = cnt_total / time_total

    # Log out the result for latter analysis.
    # pylint: disable=logging-fstring-interpolation
    LOG.info(f"Perf test average throughput is: {ops_avg} ops (operations per second)")

def _test_response(svc_call):
    """Generic response time test workflow.

    Args:
        svc_call: function to invoke the service call.
    """
    # 1. Repeat the steps below M times (M = 20 is the current setting):
    repeat_times = RESPONSE_TEST_REPEAT_TIME
    t_cost_total = 0
    for _ in range(repeat_times):
        # Start a new process to simulate an app. In the process:
        #   Begin timing.
        #   Call (one immediately after another) the CCNP API (either via SDK or
        #         request to service directly).
        #   End timing.
        #   Record the time consumption and exit.
        t_cost = _start_proc_and_wait(_timing_response, svc_call)
        t_cost_total += t_cost

    # 2. Calculate the average response time (total time divided by  M).
    t_cost_avg = t_cost_total / repeat_times
    # pylint: disable=logging-fstring-interpolation
    LOG.info(f"Perf test average response time is: {t_cost_avg / 1000000} ms (milliseconds)")

def _sdk_get_cc_measurement():
    """Using CCNP SDK to get CC measurement."""
    # Current just test the first IMR with index 0 and hash algorithm ID 12.
    CcnpSdk.inst().get_cc_measurement([0, 12])

def _sdk_get_cc_eventlog():
    """Using CCNP SDK to get CC eventlog."""
    CcnpSdk.inst().get_cc_eventlog()

def _sdk_get_cc_report():
    """Using CCNP SDK to get CC report (i.e. quote)."""
    CcnpSdk.inst().get_cc_report()

def test_svc_get_cc_measurement_throughput():
    """Test the throughput of CCNP Service get_cc_measurement."""
    _test_throughput(_sdk_get_cc_measurement)

def test_svc_get_cc_measurement_response():
    """Test the response time of CCNP Service get_cc_measurement."""
    _test_response(_sdk_get_cc_measurement)

def test_svc_get_cc_eventlog_throughput():
    """Test the throughput of CCNP Service get_cc_eventlog."""
    _test_throughput(_sdk_get_cc_eventlog)

def test_svc_get_cc_eventlog_response():
    """Test the response time of CCNP Service get_cc_eventlog."""
    _test_response(_sdk_get_cc_eventlog)

def test_svc_get_cc_report_throughput():
    """Test the throughput of CCNP Service get_cc_report."""
    _test_throughput(_sdk_get_cc_report)

def test_svc_get_cc_report_response():
    """Test the response time of CCNP Service get_cc_report."""
    _test_response(_sdk_get_cc_report)

def test_ccnp_init():
    """Test the initialization time of CCNP.
    i.e. The time cost of the initialization for CCNP Device Plugin and CCNP Service so
    they are ready for service requests.
    """
    # TODO:
    # Repeat R times (R = 20 is the current setting) and calculate the average time
    # (total times divided by R):
    #   Begin timing.
    #   Start CCNP deployment (incl. CCNP Device Plugin and CCNP Service).
    #   Polling the readiness of CCNP service until it's ready.
    #   End timing.
    #   Calculate the initialization time using end time subtracted by begin time.
