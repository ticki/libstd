use core_collections::borrow::ToOwned;
use io::{self, BufRead, BufReader, Read, Error, Result, Write, Seek, SeekFrom};
use os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use mem;
use path::{PathBuf, Path};
use string::String;
use sys_common::AsInner;
use vec::Vec;

use syscall::{open, dup, close, fpath, fstat, ftruncate, read,
              write, lseek, fsync, mkdir, rmdir, unlink};
use syscall::{O_RDWR, O_RDONLY, O_WRONLY, O_APPEND, O_CREAT, O_TRUNC, MODE_DIR, MODE_FILE, MODE_PERM, SEEK_SET, SEEK_CUR, SEEK_END, Stat};

/// A Unix-style file
#[derive(Debug)]
pub struct File {
    /// The id for the file
    fd: usize,
}

impl File {
    /// Open a new file using a path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
        let path_str = path.as_ref().as_os_str().as_inner();
        open(path_str, O_RDONLY).map(|fd| unsafe { File::from_raw_fd(fd) }).map_err(|x| Error::from_sys(x))
    }

    /// Create a new file using a path
    pub fn create<P: AsRef<Path>>(path: P) -> Result<File> {
        let path_str = path.as_ref().as_os_str().as_inner();
        open(path_str, O_CREAT | O_RDWR | O_TRUNC | 0o664).map(|fd| unsafe { File::from_raw_fd(fd) }).map_err(|x| Error::from_sys(x))
    }

    /// Duplicate the file
    pub fn dup(&self, buf: &[u8]) -> Result<File> {
        dup(self.fd, buf).map(|fd| unsafe { File::from_raw_fd(fd) }).map_err(|x| Error::from_sys(x))
    }

    /// Get information about a file
    pub fn metadata(&self) -> Result<Metadata> {
        let mut stat = Stat::default();
        try!(fstat(self.fd, &mut stat).map_err(|x| Error::from_sys(x)));
        Ok(Metadata {
            stat: stat
        })
    }

    /// Get the canonical path of the file
    pub fn path(&self) -> Result<PathBuf> {
        let mut buf: [u8; 4096] = [0; 4096];
        match fpath(self.fd, &mut buf) {
            Ok(count) => Ok(PathBuf::from(unsafe { String::from_utf8_unchecked(Vec::from(&buf[0..count])) })),
            Err(err) => Err(Error::from_sys(err)),
        }
    }

    /// Flush the file data and metadata
    pub fn sync_all(&mut self) -> Result<()> {
        fsync(self.fd).and(Ok(())).map_err(|x| Error::from_sys(x))
    }

    /// Flush the file data
    pub fn sync_data(&mut self) -> Result<()> {
        fsync(self.fd).and(Ok(())).map_err(|x| Error::from_sys(x))
    }

    /// Truncates the file
    pub fn set_len(&self, size: u64) -> Result<()> {
        ftruncate(self.fd, size as usize).and(Ok(())).map_err(|x| Error::from_sys(x))
    }
}

impl AsRawFd for File {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl FromRawFd for File {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        File {
            fd: fd
        }
    }
}

impl IntoRawFd for File {
    fn into_raw_fd(self) -> RawFd {
        let fd = self.fd;
        mem::forget(self);
        fd
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        read(self.fd, buf).map_err(|x| Error::from_sys(x))
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        write(self.fd, buf).map_err(|x| Error::from_sys(x))
    }

    fn flush(&mut self) -> Result<()> {
        fsync(self.fd).and(Ok(())).map_err(|x| Error::from_sys(x))
    }
}

impl Seek for File {
    /// Seek a given position
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let (whence, offset) = match pos {
            SeekFrom::Start(offset) => (SEEK_SET, offset as isize),
            SeekFrom::Current(offset) => (SEEK_CUR, offset as isize),
            SeekFrom::End(offset) => (SEEK_END, offset as isize),
        };

        lseek(self.fd, offset, whence).map(|position| position as u64).map_err(|x| Error::from_sys(x))
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = close(self.fd);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FileType {
    dir: bool,
    file: bool,
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.dir
    }

    pub fn is_file(&self) -> bool {
        self.file
    }

    pub fn is_symlink(&self) -> bool {
        false
    }
}

impl ::os::unix::fs::FileTypeExt for FileType {
    fn is_block_device(&self) -> bool { false }
    fn is_char_device(&self) -> bool { false }
    fn is_fifo(&self) -> bool { false }
    fn is_socket(&self) -> bool { false }
}

pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    create: bool,
    truncate: bool,
    mode: u16,
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            read: false,
            write: false,
            append: false,
            create: false,
            truncate: false,
            mode: 0,
        }
    }

    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.read = read;
        self
    }

    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.write = write;
        self
    }

    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.append = append;
        self
    }

    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.create = create;
        self
    }

    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.truncate = truncate;
        self
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<File> {
        let mut flags = 0;

        if self.read && self.write {
            flags |= O_RDWR;
        } else if self.read {
            flags |= O_RDONLY;
        } else if self.write {
            flags |= O_WRONLY;
        }

        if self.append {
            flags |= O_APPEND;
        }

        if self.create {
            flags |= O_CREAT;
        }

        if self.truncate {
            flags |= O_TRUNC;
        }

        flags |= (self.mode & MODE_PERM) as usize;

        let path_str = path.as_ref().as_os_str().as_inner();
        open(path_str, flags).map(|fd| unsafe { File::from_raw_fd(fd) }).map_err(|x| Error::from_sys(x))
    }
}

impl ::os::unix::fs::OpenOptionsExt for OpenOptions {
    fn mode(&mut self, mode: u32) -> &mut Self {
        self.mode = mode as u16;
        self
    }
}

pub struct Metadata {
    stat: Stat
}

impl Metadata {
    pub fn file_type(&self) -> FileType {
        FileType {
            dir: self.stat.st_mode & MODE_DIR == MODE_DIR,
            file: self.stat.st_mode & MODE_FILE == MODE_FILE
        }
    }

    pub fn is_dir(&self) -> bool {
        self.stat.st_mode & MODE_DIR == MODE_DIR
    }

    pub fn is_file(&self) -> bool {
        self.stat.st_mode & MODE_FILE == MODE_FILE
    }

    pub fn len(&self) -> u64 {
        self.stat.st_size
    }

    pub fn permissions(&self) -> Permissions {
        Permissions {
            mode: self.stat.st_mode & MODE_PERM
        }
    }
}

impl ::os::unix::fs::MetadataExt for Metadata {
    fn mode(&self) -> u32 {
        self.stat.st_mode as u32
    }

    fn uid(&self) -> u32 {
        self.stat.st_uid
    }

    fn gid(&self) -> u32 {
        self.stat.st_gid
    }

    fn size(&self) -> u64 {
        self.stat.st_size
    }
}

pub struct Permissions {
    mode: u16
}

impl Permissions {
    pub fn readonly(&self) -> bool {
        self.mode & 0o222 == 0
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        if readonly {
            self.mode &= !0o222;
        } else {
            self.mode |= 0o222;
        }
    }
}

impl ::os::unix::fs::PermissionsExt for Permissions {
    fn mode(&self) -> u32 {
        self.mode as u32
    }

    fn set_mode(&mut self, mode: u32) {
        self.mode = mode as u16;
    }

    fn from_mode(mode: u32) -> Self {
        Permissions {
            mode: mode as u16
        }
    }
}

pub struct DirEntry {
    path: PathBuf,
}

impl DirEntry {
    pub fn file_name(&self) -> &Path {
        unsafe { mem::transmute(self.path.file_name().unwrap().to_str().unwrap()) }
    }

    pub fn file_type(&self) -> Result<FileType> {
        self.metadata().map(|metadata| metadata.file_type())
    }

    pub fn metadata(&self) -> Result<Metadata> {
        metadata(&self.path)
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

pub struct ReadDir {
    path: PathBuf,
    file: BufReader<File>,
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry>;
    fn next(&mut self) -> Option<Result<DirEntry>> {
        let mut name = String::new();
        match self.file.read_line(&mut name) {
            Ok(0) => None,
            Ok(_) => {
                if name.ends_with('\n') {
                    name.pop();
                }

                let mut path = self.path.clone();
                path.push(name);
                Some(Ok(DirEntry {
                    path: path
                }))
            },
            Err(err) => Some(Err(err))
        }
    }
}

/// Find the canonical path of a file
pub fn canonicalize<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    match File::open(path) {
        Ok(file) => {
            match file.path() {
                Ok(realpath) => Ok(realpath),
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}

/// Get information about a file
pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    try!(File::open(path)).metadata()
}

/// Get information about a file without following symlinks
/// Warning: Redox does not currently support symlinks
pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    metadata(path)
}

/// Create a new directory, using a path
/// The default mode of the directory is 775
pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path_str = path.as_ref().as_os_str().as_inner();
    mkdir(path_str, 0o775).and(Ok(())).map_err(|x| Error::from_sys(x))
}

/// Recursively create a directory and all of its parent components if they are missing.
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        try!(create_dir_all(&parent));
    }
    if let Err(_err) = metadata(&path) {
        try!(create_dir(&path));
    }
    Ok(())
}

/// Copy the contents of one file to another
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64> {
    let mut infile = try!(File::open(from));
    let mut outfile = try!(File::create(to));
    io::copy(&mut infile, &mut outfile)
}

/// Rename a file or directory to a new name
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    try!(copy(Path::new(from.as_ref()), to));
    remove_file(from)
}

/// Return an iterator over the entries within a directory
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir> {
    let path_buf = path.as_ref().to_owned();
    File::open(&path_buf).map(|file| ReadDir { path: path_buf, file: BufReader::new(file) })
}

/// Removes an existing, empty directory
pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path_str = path.as_ref().as_os_str().as_inner();
    rmdir(path_str).and(Ok(())).map_err(|x| Error::from_sys(x))
}

/// Removes a directory at this path, after removing all its contents. Use carefully!
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    for child in try!(read_dir(&path)) {
        let child = try!(child);
        if try!(child.file_type()).is_dir() {
            try!(remove_dir_all(&child.path()));
        } else {
            try!(remove_file(&child.path()));
        }
    }
    remove_dir(path)
}

/// Removes a file from the filesystem
pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let path_str = path.as_ref().as_os_str().as_inner();
    unlink(path_str).and(Ok(())).map_err(|x| Error::from_sys(x))
}
