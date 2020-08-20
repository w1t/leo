setup_color() {
	# Only use colors if connected to a terminal
	if [ -t 1 ]; then
		YELLOW=$(printf '\033[33m')
		RESET=$(printf '\033[m')
	else
		YELLOW=""
		RESET=""
	fi
}

install() {
  echo "Fetching the latest ${YELLOW}Leo${RESET} binary..."

  curl -sL "$(curl -L -s https://api.github.com/repos/AleoHQ/leo/releases/latest |
    jq -r '.assets[] | select(.name | contains("-x86_64-unknown-linux-gnu.zip")).browser_download_url')" |
      tar -xzv

  echo "Download complete."
}

setup_color
install