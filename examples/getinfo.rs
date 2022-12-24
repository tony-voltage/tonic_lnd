// This example only fetches and prints the node info to the standard output similarly to
// `lncli getinfo`.
//
// The program accepts three arguments: address, cert file, macaroon file
// Example run: `cargo run --features=lightningrpc --example getinfo <address> <tls.cert> <file.macaroon>`

#[tokio::main]
#[cfg(feature = "lightningrpc")]
async fn main() {
    use std::net::SocketAddr;

    let mut args = std::env::args_os();
    args.next().expect("not even zeroth arg given");
    let address: SocketAddr = args
        .next()
        .expect("missing arguments: address, cert file, macaroon file")
        .into_string()
        .expect("address is not UTF-8")
        .parse()
        .expect("address is not SocketAddr");
    let cert_file: String = args
        .next()
        .expect("missing arguments: cert file, macaroon file")
        .into_string()
        .expect("cert_file is not UTF-8");
    let macaroon_file: String = args
        .next()
        .expect("missing argument: macaroon file")
        .into_string()
        .expect("macaroon_file is not UTF-8");

    // Connecting to LND requires only address, cert file, macaroon file
    let mut client = tonic_lnd::connect(address, cert_file, macaroon_file)
        .await
        .expect("failed to connect");

    let info = client
        .lightning()
        // All calls require at least empty parameter
        .get_info(tonic_lnd::lnrpc::GetInfoRequest {})
        .await
        .expect("failed to get info");

    // We only print it here, note that in real-life code you may want to call `.into_inner()` on
    // the response to get the message.
    println!("{:#?}", info);
}
