#!/bin/bash
set -o errexit
: '
This is an E2E test script.
If you want to run the script , please run the ci-setup.sh script in advance to set up the ci test environment.

Please enter parameters to select the test case you want to run:
-a, run all test cases.
-s py/go, run python/golang sdk test cases, such as "-s py" means running python test cases.
'
PY_WORK_DIR='test/e2e-test/py-test'
GO_WORK_DIR='test/e2e-test/go-test/test-cases'
PY_FLAG=false
GO_FLAG=false


while getopts "as:" opt_name
do
    case $opt_name in
        a) PY_FLAG=true
           GO_FLAG=true
           echo "Run all test cases" 
           ;;
        s) IFS=','
           read -ra array <<< "${OPTARG}"
           for var in "${array[@]}"
           do
                if [[ $var =~ "py" ]]
                then
                        PY_FLAG=true
                        echo "Run py test cases"
                elif [[ $var =~ "go" ]]
                then
                        GO_FLAG=true
                        echo "Run go test cases"
                fi
           done
           ;;
        ?) echo "Invalid input"
　　　　　 ;;
    esac
done




for i in {1..3}
do
    POD_NAME=$(kubectl get po | grep ccnp-test-node | grep Running | awk '{ print $1 }')
    if [[ -z "$POD_NAME" ]]
    then
        sleep 2
        echo "Retrying $i time ..."
    else
        break
    fi
done



#Run python E2E test cases
if $PY_FLAG
then
    kubectl exec -it "$POD_NAME" -- pytest -v ${PY_WORK_DIR}
fi

#Run golang E2E test cases
if $GO_FLAG
then
    kubectl exec -it "$POD_NAME" -- bash  -c "pushd  ${GO_WORK_DIR};go test -v;popd "
fi

