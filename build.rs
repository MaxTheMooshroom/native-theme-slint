use anyhow::Result;

fn main() -> Result<()> {
    let config = slint_build::CompilerConfiguration::new()
        .as_library("SystemTheme")
        .rust_module("slint_modules");

    slint_build::compile_with_config("ui/system-components.slint", config)
        .map_err(Into::into)
}