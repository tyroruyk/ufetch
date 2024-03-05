#!/bin/bash

readonly DIR="/usr/bin"

RED='\033[1;31m'
GREEN='\033[0;32m'
NC='\033[0m'

installation() {
    echo "Installing Ufetch..."

    echo "Building..."
    cargo build --release

    echo "Copying Files..."
    sudo cp target/release/ufetch ${DIR}/ufetch

    echo "${GREEN}Installation Done!{$NC}"
}

uninstallation() {
    echo "Removing Files..."

    sudo rm ${DIR}/ufetch
    echo "${RED}Uninstallation Done!{$NC}"
}

installation_prompt() {
    while true; do
        read -r -p "Install ufetch? [Y/N] " yn
        case $yn in
        [Yy]*)
            installation
            break
            ;;
        [Nn]*) break ;;
        esac
    done
}

main() {
    if test -f "${DIR}/findex"; then
        while true; do
            read -r -p "Found existing installation. Do you want to uninstall it? [Y/N] " yn
            case $yn in
            [Yy]*)
                uninstallation
                installation_prompt
                exit
                ;;
            [Nn]*) exit ;;
            esac
        done
    fi
    installation
}

main
