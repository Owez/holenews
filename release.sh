echo "Starting release process.."
cargo build --release
mkdir ./holenews-release/
mv ./target/release/holenews ./holenews-release/
strip ./holenews-release/holenews
cp -r ./templates/ ./holenews-release/
cp -r ./static/ ./holenews-release/
echo "Finished release, see the holenews-release/ directory"
