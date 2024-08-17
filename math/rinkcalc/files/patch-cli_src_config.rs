--- cli/src/config.rs.orig	2024-08-17 09:10:37 UTC
+++ cli/src/config.rs
@@ -290,6 +290,9 @@ pub fn load(config: &Config) -> Result<Context> {
         search_path.push(prefix.into());
     }
 
+    // FreeBSD share
+    search_path.push(PathBuf::from("/usr/local/share/rink"));
+
     // Read definitions.units
     let units = read_from_search_path("definitions.units", &search_path)
         .or_else(|err| DEFAULT_FILE.map(ToOwned::to_owned).ok_or(err).wrap_err("Rink was not built with a bundled definitions.units file, and one was not found in the search path."))?;
