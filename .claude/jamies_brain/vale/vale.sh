#curl -L --progress-bar \
#  "https://github.com/vale-cli/vale/releases/download/v3.13.0/vale_3.13.0_Linux_arm64.tar.gz" \
#  -o /tmp/vale.tar.gz

file /tmp/vale.tar.gz

mkdir -p ~/.local/bin
tar -xzf /tmp/vale.tar.gz -C ~/.local/bin vale
rm /tmp/vale.tar.gz
vale --version
