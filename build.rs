fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=protos/");

    tonic_build::compile_protos("protos/types.proto")?;

    tonic_build::configure().build_client(false).compile(
        &["protos/recommender/recommender.proto"],
        &["protos"],
    )?;

    tonic_build::configure().build_server(false).compile(
        &[
            "protos/database/users.proto",
            "protos/database/alerts.proto",
            "protos/database/ratings.proto",
        ],
        &["protos"],
    )?;

    Ok(())
}
