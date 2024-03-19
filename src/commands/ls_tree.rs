enum Kind{
  Blob,
  Tree
}

pub fn invoke(name_only: bool) -> anyhow::Result<()> {
  anyhow::ensure!(name_only, "only --name-only is supported for now");
  
  Ok(())
}