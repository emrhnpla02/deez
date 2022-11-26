pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
  std::path::Path::new(filename)
    .extension()
    .and_then(std::ffi::OsStr::to_str)
}
