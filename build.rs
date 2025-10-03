use tonic_prost_build::configure;

fn main() {
    configure()
        .compile_protos(&["proto/execute.proto"], &["proto"])
        .expect("Cannot compile proto");
}
