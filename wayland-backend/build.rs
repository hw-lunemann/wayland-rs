fn main() {
    // build the client shim
    cc::Build::new().file("src/sys/client_impl/log_shim.c").compile("log_shim_client");
    println!("cargo:rerun-if-changed=src/sys/client_impl/log_shim.c");
    // build the server shim
    cc::Build::new().file("src/sys/server_impl/log_shim.c").compile("log_shim_server");
    println!("cargo:rerun-if-changed=src/sys/server_impl/log_shim.c");
}
