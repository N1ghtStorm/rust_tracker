cd ./tracker_enricher/
cargo run --release &
cd ../tracker_facade/
cargo run --release &
cd  ../tracker_payloads_keeper
cargo run --release &
cd ../tracker_frontend
npm start &
