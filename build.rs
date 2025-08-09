fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile_protos(&["src/api/grpc/proto/helloworld.proto"], &["proto"])
        .unwrap();
}
