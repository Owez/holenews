# start msg
echo "Starting release process.."

# build binary
cargo build --release
mkdir ./holenews-release/
mv ./target/release/holenews ./holenews-release/
strip ./holenews-release/holenews

# add content
cp -r ./templates/ ./holenews-release/
cp -r ./static/ ./holenews-release/
cp -r ./migrations/ ./holenews-release/

# database
cd ./holenews-release/
sqlx database create
sqlx migrate run
rm -rf ./migrations/
cd ..

# finish msg
echo "Finished release, see the holenews-release/ directory"
