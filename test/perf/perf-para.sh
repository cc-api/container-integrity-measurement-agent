#!/bin/bash
# Script to execute CCNP example pod

set -e

number=1
is_eventlog=false
is_measurement=false
is_quote=false
command_eventlog="parallel perf_eventlog ::: "
command_measurement="parallel perf_measurement ::: "
command_quote="parallel perf_quote ::: "

usage() {
    cat << EOM
usage: $(basename "$0") [OPTION]...
    -n Number of pods to run parallel perf testing
    -e Run perf test for get eventlog
    -m Run perf test for get measurement
    -r Run perf test for get quote
EOM
    exit 1
}

process_args() {
    while getopts ":n:emrh" option; do
            case "${option}" in
                n) number=${OPTARG};;
                e) is_eventlog=true;;
                m) is_measurement=true;;
                r) is_quote=true;;
                h) usage;;
                *) echo "Invalid option: -${OPTARG}" >&2
                usage
                ;;
            esac
        done
}

# generate a series of perf scripts
gen_commands() {
    for ((i=1; i<=number; i++)); do
        pod_name=$(kubectl get po -n ccnp | grep -i ccnp-perf-example-$i- | grep Running | awk '{ print $1 }')

        echo "kubectl exec -it $pod_name -n ccnp -- python3 py_perf.py -e" > /tmp/ccnp-perf-eventlog-$i.sh
        chmod +x /tmp/ccnp-perf-eventlog-$i.sh

        echo "kubectl exec -it $pod_name -n ccnp -- python3 py_perf.py -m" > /tmp/ccnp-perf-measurement-$i.sh
        chmod +x /tmp/ccnp-perf-measurement-$i.sh

        echo "kubectl exec -it $pod_name -n ccnp -- python3 py_perf.py -r" > /tmp/ccnp-perf-quote-$i.sh
        chmod +x /tmp/ccnp-perf-quote-$i.sh

        command_eventlog+="/tmp/ccnp-perf-eventlog-$i.sh "
        command_measurement+="/tmp/ccnp-perf-measurement-$i.sh "
        command_quote+="/tmp/ccnp-perf-quote-$i.sh "
    done
}

# Function to execute a script and measure time
perf_eventlog() {
    script_name=$1
    echo "Running $script_name..."
    # Use time to measure execution time
    output=perf_output_eventlog-$(date +"%Y%m%d_%H%M%S")
    /usr/bin/time -f "$script_name took %E" bash "$script_name" >> "$output" 2>&1
}

perf_measurement() {
    script_name=$1
    echo "Running $script_name..."
    # Use time to measure execution time
    output=perf_output_measurement-$(date +"%Y%m%d_%H%M%S")
    /usr/bin/time -f "$script_name took %E" bash "$script_name" >> "$output"  2>&1
}

perf_quote() {
    script_name=$1
    echo "Running $script_name..."
    # Use time to measure execution time
    output=perf_output_quote-$(date +"%Y%m%d_%H%M%S")
    /usr/bin/time -f "$script_name took %E" bash "$script_name" >> "$output" 2>&1
}

run_perf_eventlog() {
    export -f perf_eventlog
    # Start time
    start=$(date +%s)
    $command_eventlog

    # End time
    end=$(date +%s)

    # Calculate total duration
    duration=$((end - start))
    echo "Parallel - Total execution time for all perf_eventlog scripts: $duration seconds"
}

run_perf_measurement() {
    export -f perf_measurement
    # Start time
    start=$(date +%s)
    $command_measurement

    # End time
    end=$(date +%s)

    # Calculate total duration
    duration=$((end - start))
    echo "Parallel - Total execution time for all perf_measurement scripts: $duration seconds"
}

run_perf_quote() {
    export -f perf_quote
    # Start time
    start=$(date +%s)
    $command_quote

    # End time
    end=$(date +%s)

    # Calculate total duration
    duration=$((end - start))
    echo "Parallel - Total execution time for all perf_quote scripts: $duration seconds"
}

process_args "$@"

echo ""
echo "-------------------------"
echo "number of pods for perf testing: ${number}"
echo "run perf_eventlog: ${is_eventlog}"
echo "run perf_measurement: ${is_measurement}"
echo "run perf_quote: ${is_quote}"
echo "-------------------------"
echo ""

gen_commands

if [[ $is_eventlog == true ]]; then
    run_perf_eventlog
fi

if [[ $is_measurement == true ]]; then
    run_perf_measurement
fi

if [[ $is_quote == true ]]; then
    run_perf_quote
fi
