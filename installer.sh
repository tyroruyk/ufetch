#!/bin/bash

readonly DIR="/usr/bin"

installation() {
	echo "Installing Ufetch..."

	echo "Building..."
	cargo build --release

	echo "Copying Files..."
	sudo cp target/release/ufetch ${DIR}/ufetch

	echo "Installation Done!"
}

uninstallation() {
	echo "Removing Files..."

	sudo rm ${DIR}/ufetch
	echo "Uninstallation Done"
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
	if test -f "${DIR}/ufetch"; then
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
