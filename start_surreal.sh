FILE=surrealdb/target/release/surreal
if [ -f "$FILE" ]; then
    echo "surrealdb is built with release config. Starting it"
else 
    echo "$FILE is not built. Build it first."
    exit 1
fi

bash -c "$FILE start --log debug --user root --pass root memory"
