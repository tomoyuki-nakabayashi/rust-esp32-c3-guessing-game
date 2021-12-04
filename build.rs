use embuild::build::{CfgArgs, LinkArgs};

fn main() -> anyhow::Result<()> {
    LinkArgs::output_propagated("ESP_IDF")?;
    let cfg = CfgArgs::try_from_env("ESP_IDF")?;
    cfg.output();

    Ok(())
}
