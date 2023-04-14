fn main() -> Result<(), Box<dyn std::error::Error>> {
    vergen::EmitBuilder::builder()
        .all_cargo()
        .all_rustc()
        .git_sha(false)
        .emit()?;

    Ok(())
}
