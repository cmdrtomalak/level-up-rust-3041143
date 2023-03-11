use std::path;
use std::os::unix::fs::PermissionsExt;
use libc;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        let meta = self.metadata().unwrap().permissions();
        let mode = meta.mode();

        // Check if any read bit is set
        mode & (libc::S_IRUSR | libc::S_IRGRP | libc::S_IROTH) != 0
    }

    fn is_writeable(&self) -> bool {
        let meta = self.metadata().unwrap().permissions();
        let mode = meta.mode();

        // Check if any write bit is set
        mode & (libc::S_IWUSR | libc::S_IWGRP | libc::S_IWOTH) != 0
    }

    fn exists(&self) -> bool {
        self.exists()
    }
}

fn main() {
    // 
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}
