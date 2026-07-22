#[cfg(feature = "openapi")]
fn main() -> anyhow::Result<()> {
    let mut check = false;
    let mut include_exp = false;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--check" => check = true,
            "--include-exp" => include_exp = true,
            other => anyhow::bail!("Unsupported argument: {other}"),
        }
    }

    fastnear_api_server_rs::openapi::generate(check, include_exp)
}

#[cfg(not(feature = "openapi"))]
fn main() {
    eprintln!("This binary requires the `openapi` feature.");
    std::process::exit(1);
}
