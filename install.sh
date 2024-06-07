#!/usr/bin/env bash

get_ver() {
	local ver
	ver=$(curl -s "https://api.github.com/repos/joeldotdias/lace/releases/latest" | grep -Po "\"tag_name\": \"v\K[^\"]*")
	echo "$ver"
}

version=$(get_ver)

curl -Lo lace.tar.gz "https://github.com/joeldotdias/lace/releases/latest/download/lace_${version}_linux_x86_64.tar.gz"
tar xf lace.tar.gz lace
install -Dm 755 lace -t "$HOME/.local/bin/"

# cleanup
rm lace.tar.gz lace
