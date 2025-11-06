use std::path::{Path, PathBuf};

pub trait PathExt {
    /// Append a path to [self], returning [None] if the resulting path would not fall under
    /// [self].
    ///
    /// `/foo` and `bar` -> `/foo/bar`
    /// `/foo` and `/hop` -> [None]
    /// `/foo/bar` and `/foo/bar/baz` -> `/foo/bar/baz`
    fn join_inside<P: AsRef<Path>>(&self, other: P) -> Option<PathBuf>;
}

impl PathExt for PathBuf {

    fn join_inside<P: AsRef<Path>>(&self, other: P) -> Option<PathBuf> {
        let other = other.as_ref();
        if other.is_absolute() {
            let is_inside_self = pathdiff::diff_paths(other, self).is_some();
            if is_inside_self {
                Some(other.to_path_buf())
            } else {
                None // Or an error
            }
        } else {
            Some(self.join(other))
        }
    }

}
