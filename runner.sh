#!/bin/bash

countdown() {
    local GREEN='\033[0;32m'
    local RED='\033[0;31m'
    local YELLOW='\033[0;33m'
    local RESET='\033[0m'
    local min=$1
    local sec=$2
    
    tput civis
    echo -ne "${GREEN}"
    
    while [ $min -ge 0 ]; do
        while [ $sec -ge 0 ]; do
            if [ "$min" -eq "0" ]; then
                echo -ne "${YELLOW}"
            fi
            if [ "$min" -eq "0" ] && [ "$sec" -le "10" ]; then
                echo -ne "${RED}"
            fi
            echo -ne " > snoozing $(printf "%02d" $min):$(printf "%02d" $sec)\033[0K\r"
            let "sec=sec-1"
            sleep 1
        done
        if [ $min -gt 0 ]; then
            sec=59
            let "min=min-1"
        fi
    done
    
    echo -e "${RESET}"
    tput cnorm
}

# since you can only redeem after every 10 minutes a sleepof 630 is necessary
while true; do
    # run you executable here with arguments
    countdown 1 0
done



