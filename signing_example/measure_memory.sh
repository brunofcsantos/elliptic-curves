clear
(return 0 2>/dev/null) || { echo "This script must be sourced, not executed."; return; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
START_DIR=$PWD

cd $SCRIPT_DIR
echo $SCRIPT_DIR
cargo +nightly build --release


# Signing with key P256
FOLDER="$SCRIPT_DIR/../target/criterion/P256/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/sign_memory_p256.txt" "$SCRIPT_DIR/../target/release/sign_memory_p256" > "$FOLDER/sign_memory_p256.txt"
echo "Wrote to $FOLDER/sign_memory_p256.txt"

# Signing with key P384
FOLDER="$SCRIPT_DIR/../target/criterion/P384/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/sign_memory_p384.txt" "$SCRIPT_DIR/../target/release/sign_memory_p384" > "$FOLDER/sign_memory_p384.txt"
echo "Wrote to $FOLDER/sign_memory_p384.txt"

# Signing with key P521
FOLDER="$SCRIPT_DIR/../target/criterion/P521/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/sign_memory_p521.txt" "$SCRIPT_DIR/../target/release/sign_memory_p521" > "$FOLDER/sign_memory_p521.txt"
echo "Wrote to $FOLDER/sign_memory_p521.txt"



# Verifying with key P256
FOLDER="$SCRIPT_DIR/../target/criterion/P256/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/verify_memory_p256.txt" "$SCRIPT_DIR/../target/release/verify_memory_p256" > "$FOLDER/verify_memory_p256.txt"
echo "Wrote to $FOLDER/verify_memory_p256.txt"

# Verifying with key P384
FOLDER="$SCRIPT_DIR/../target/criterion/P384/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/verify_memory_p384.txt" "$SCRIPT_DIR/../target/release/verify_memory_p384" > "$FOLDER/verify_memory_p384.txt"
echo "Wrote to $FOLDER/verify_memory_p384.txt"

# Verifying with key P521
FOLDER="$SCRIPT_DIR/../target/criterion/P521/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/verify_memory_p521.txt" "$SCRIPT_DIR/../target/release/verify_memory_p521" > "$FOLDER/verify_memory_p521.txt"
echo "Wrote to $FOLDER/verify_memory_p521.txt"


# Creating key P256
FOLDER="$SCRIPT_DIR/../target/criterion/P256/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/create_keys_p256.txt" "$SCRIPT_DIR/../target/release/create_keys_p256" > "$FOLDER/create_keys_p256.txt"
echo "Wrote to $FOLDER/create_keys_p256.txt"

# Creating key P384
FOLDER="$SCRIPT_DIR/../target/criterion/P384/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/create_keys_p384.txt" "$SCRIPT_DIR/../target/release/create_keys_p384" > "$FOLDER/create_keys_p384.txt"
echo "Wrote to $FOLDER/create_keys_p384.txt"

# Creating key P521
FOLDER="$SCRIPT_DIR/../target/criterion/P521/memory"
mkdir -p "$FOLDER"
valgrind --tool=massif --stacks=yes --massif-out-file="$FOLDER/create_keys_p521.txt" "$SCRIPT_DIR/../target/release/create_keys_p521" > "$FOLDER/create_keys_p521.txt"
echo "Wrote to $FOLDER/create_keys_p521.txt"