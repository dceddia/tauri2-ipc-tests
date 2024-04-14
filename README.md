# Tauri 2 IPC sending/receiving data

A small test of transferring binary data via IPC.

There is probably a faster way than using `emit` from Rust, because in DEV mode, that takes ~400ms+ on my M1 whereas the JS-initiated pull takes ~20-40ms. In release mode, the `emit` gets 10x faster. So I suspect there's a faster way to initiate transfers from the Rust side.
