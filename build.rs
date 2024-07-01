fn main() {
    let default_conf = tonic_build::configure().build_client(false);
    default_conf
        .clone()
        .out_dir("./crates/mada-immo-admin-tonic/src/proto")
        .compile(&["./proto/admin.proto"], &["./proto"])
        .unwrap();
    default_conf
        .clone()
        .out_dir("./crates/mada-immo-client-tonic/src/proto")
        .compile(&["./proto/client.proto"], &["./proto"])
        .unwrap();
    default_conf
        .clone()
        .out_dir("./crates/mada-immo-proprio-tonic/src/proto")
        .compile(&["./proto/proprietaire.proto"], &["./proto"])
        .unwrap();
}
