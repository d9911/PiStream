use std::{env, error::Error, fs, path::PathBuf, process::Command};
fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=assets/icons/pistream.ico");
    println!("cargo:rerun-if-changed=packaging/windows/pistream.rc");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    if !env::var("HOST")?.contains("windows") {
        println!("cargo:warning=Build the final Windows package on Windows to embed its icon.");
        return Ok(());
    }
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or("Missing project path")?);
    let out = PathBuf::from(env::var_os("OUT_DIR").ok_or("Missing output directory")?);
    fs::copy(
        root.join("assets/icons/pistream.ico"),
        out.join("pistream.ico"),
    )?;
    let gnu = env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu");
    let output = if gnu {
        "pistream-resource.o"
    } else {
        "pistream.res"
    };
    let mut command = Command::new(if gnu { "windres" } else { "rc.exe" });
    command.current_dir(&out);
    if gnu {
        command
            .arg(root.join("packaging/windows/pistream.rc"))
            .args(["-O", "coff", "-o", output]);
    } else {
        command
            .args(["/nologo", "/fo", output])
            .arg(root.join("packaging/windows/pistream.rc"));
    }
    if !command.status()?.success() {
        return Err("Windows icon resource compilation failed".into());
    }
    println!(
        "cargo:rustc-link-arg-bin=PiStreamTerminal={}",
        out.join(output).display()
    );
    Ok(())
}
