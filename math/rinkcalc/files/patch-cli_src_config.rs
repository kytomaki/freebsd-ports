--- cli/src/config.rs.orig	2024-08-17 08:57:14 UTC
+++ cli/src/config.rs
@@ -289,7 +289,7 @@ pub fn load(config: &Config) -> Result<Context> {
     if let Some(prefix) = option_env!("RINK_PATH") {
         search_path.push(prefix.into());
     }

+    search_path.push(PathBuf::from("/usr/local/share/rink")
+
     // Read definitions.units
     let units = read_from_search_path("definitions.units", &search_path)
         .or_else(|err| DEFAULT_FILE.map(ToOwned::to_owned).ok_or(err).wrap_err("Rink was not built with a bundled definitions.units file, and one was not found in the search path."))?;
