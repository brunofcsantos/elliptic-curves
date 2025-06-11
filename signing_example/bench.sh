clear
(return 0 2>/dev/null) || { echo "This script must be sourced, not executed."; return; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
START_DIR=$PWD

cd $SCRIPT_DIR
echo $SCRIPT_DIR
cargo +nightly bench
