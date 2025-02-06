#!/usr/bin/env bash
set -euo pipefail
R=$(echo ${EPOCHREALTIME/[^0-9]/} | base64 | tail -c 10)
R=${R%o=}
DIR_SUPER=/tmp/nit-concurrent/
DIR=$DIR_SUPER/$R
mkdir -p $DIR
PIDS=()
declare -A JOBS=()

YELLOW='\033[0;33m'
RESET='\033[0m'
INTENSE_WHITE='\033[1;37m'
INTENSE_BLACK='\033[1;30m'
GREEN='\033[0;32m'
ITALIC='\033[3m'
RED='\033[0;31m'

function add_proc {
	$(cargo $2 > "$DIR/out_$1" 2>&1 || { ERR=$? && echo $ERR > "$DIR/exit_$1" && exit $ERR; }) & {
		PID=$!;
		PIDS+=($PID);
		JOBS[$PID]=$1;
	}
}

echo -e "=== ${INTENSE_WHITE}Running semi-concurrent checks...${RESET} ==="

for release in nightly stable; do
	add_proc "[${release^}] Clippy"         "+${release} clippy --color always"
	add_proc "[${release^}] Test"           "+${release} test   --color always --quiet --examples"
	add_proc "[${release^}] Test (Release)" "+${release} test   --color always --quiet --examples --release"
done

ERRORED=false
for PID in "${PIDS[@]}"; do
	if wait -n; then
		echo -e "${GREEN}Finished:${RESET} ${JOBS[$PID]} ${INTENSE_BLACK}($PID)${RESET}"
	else
		ERRORED=true
		EXIT_CODE=$?
		for PID in "${PIDS[@]}"; do
			kill "$PID" 2> /dev/null || :
		done
		for PID in "${PIDS[@]}"; do
			JOB=${JOBS[$PID]}
			if [ -f "$DIR/exit_$JOB" ]; then
				PEXIT_CODE=$(cat "$DIR/exit_$JOB")
				if [[ $PEXIT_CODE -ne 0 && $PEXIT_CODE -ne -1 ]]; then
					echo -e "$RED==$RESET ${INTENSE_WHITE}$JOB${RESET} $RED==$RESET"
					if [ -f "$DIR/out_$JOB" ]; then
						cat "$DIR/out_$JOB"
						echo -e "$YELLOW*$RESET ${ITALIC}The normal ${RESET}check.sh ${ITALIC}script may provide a more in-context view of the cause of the issue.$RESET"
					else
						echo "... (no output?) ..."
					fi
				fi
			fi
		done
		(sleep 5 && { rm -rf $DIR; [ -z "$(ls -A $DIR_SUPER)" ] && rmdir $DIR_SUPER; }) &
		exit $EXIT_CODE
	fi
done

echo -e " ${GREEN}✓${RESET} ${INTENSE_WHITE}No preliminary critical issues${RESET}, but there may things that were not logged."
