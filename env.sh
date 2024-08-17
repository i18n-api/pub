set -e
DIR=$(dirname "${BASH_SOURCE[0]}")

if [ ! -d $DIR/node_modules ]; then
  bun i
fi
