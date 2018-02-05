use std::fs::Metadata;

pub fn get_mode(meta: &Box<Metadata>) -> String {
    let mode: u32;
    let mut mode_str = String::new();

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        mode = meta.mode();
        mode_str.push_str(&get_mode_unix(mode));
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        mode = meta.file_attributes();
        mode_str.push_str(&get_mode_windows(mode));
    }

    mode_str
}

fn get_mode_unix(mode: u32) -> String {
    let mut s = String::new();

    // user

    if mode & S_IRUSR == S_IRUSR {
        s.push('r')
    } else {
        s.push('-');
    }

    if mode & S_IWUSR == S_IWUSR {
        s.push('w')
    } else {
        s.push('-');
    }

    if mode & S_IXUSR == S_IXUSR {
        s.push('x')
    } else {
        s.push('-');
    }

    // group

    if mode & S_IRGRP == S_IRGRP {
        s.push('r')
    } else {
        s.push('-');
    }

    if mode & S_IWGRP == S_IWGRP {
        s.push('w')
    } else {
        s.push('-');
    }

    if mode & S_IXGRP == S_IXGRP {
        s.push('x')
    } else {
        s.push('-');
    }

    // other

    if mode & S_IROTH == S_IROTH {
        s.push('r')
    } else {
        s.push('-');
    }

    if mode & S_IWOTH == S_IWOTH {
        s.push('w')
    } else {
        s.push('-');
    }

    if mode & S_IXOTH == S_IXOTH {
        s.push('x')
    } else {
        s.push('-');
    }

    s
}

fn get_mode_unix_int(meta: &Box<Metadata>) -> Option<u32> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Some(meta.mode())
    }

    #[cfg(windows)]
    {
        None
    }
}

pub fn user_read(meta: &Box<Metadata>) -> bool {
    match get_mode_unix_int(meta) {
        Some(mode) => mode & S_IRUSR == S_IRUSR,
        None => false
    }
}

pub fn user_write(meta: &Box<Metadata>) -> bool {
    match get_mode_unix_int(meta) {
        Some(mode) => mode & S_IWUSR == S_IWUSR,
        None => false
    }
}

pub fn user_exec(meta: &Box<Metadata>) -> bool {
    match get_mode_unix_int(meta) {
        Some(mode) => mode & S_IXUSR == S_IXUSR,
        None => false
    }
}

pub fn group_read(meta: &Box<Metadata>) -> bool {
    match get_mode_unix_int(meta) {
        Some(mode) => mode & S_IRGRP == S_IRGRP,
        None => false
    }
}

pub fn group_write(meta: &Box<Metadata>) -> bool {
    match get_mode_unix_int(meta) {
        Some(mode) => mode & S_IWGRP == S_IWGRP,
        None => false
    }
}

pub fn group_exec(meta: &Box<Metadata>) -> bool {
    match get_mode_unix_int(meta) {
        Some(mode) => mode & S_IXGRP == S_IXGRP,
        None => false
    }
}

pub fn other_read(meta: &Box<Metadata>) -> bool {
    match get_mode_unix_int(meta) {
        Some(mode) => mode & S_IROTH == S_IROTH,
        None => false
    }
}

pub fn other_write(meta: &Box<Metadata>) -> bool {
    match get_mode_unix_int(meta) {
        Some(mode) => mode & S_IWOTH == S_IWOTH,
        None => false
    }
}

pub fn other_exec(meta: &Box<Metadata>) -> bool {
    match get_mode_unix_int(meta) {
        Some(mode) => mode & S_IXOTH == S_IXOTH,
        None => false
    }
}

const S_IRUSR: u32 = 400;
const S_IWUSR: u32 = 200;
const S_IXUSR: u32 = 100;

const S_IRGRP: u32 = 40;
const S_IWGRP: u32 = 20;
const S_IXGRP: u32 = 10;

const S_IROTH: u32 = 4;
const S_IWOTH: u32 = 2;
const S_IXOTH: u32 = 1;

const S_ISUID: u32 = 4000;
const S_ISGID: u32 = 2000;
const S_ISVTX: u32 = 1000;

fn get_mode_windows(mode: u32) -> String {
    let mut v = vec![];

    if mode & FILE_ATTRIBUTE_ARCHIVE == FILE_ATTRIBUTE_ARCHIVE {
        v.push("Archive");
    }

    if mode & FILE_ATTRIBUTE_COMPRESSED == FILE_ATTRIBUTE_COMPRESSED {
        v.push("Compressed");
    }

    if mode & FILE_ATTRIBUTE_DEVICE == FILE_ATTRIBUTE_DEVICE {
        v.push("Device");
    }

    if mode & FILE_ATTRIBUTE_DIRECTORY == FILE_ATTRIBUTE_DIRECTORY {
        v.push("Directory");
    }

    if mode & FILE_ATTRIBUTE_ENCRYPTED == FILE_ATTRIBUTE_ENCRYPTED {
        v.push("Encrypted");
    }

    if mode & FILE_ATTRIBUTE_HIDDEN == FILE_ATTRIBUTE_HIDDEN {
        v.push("Hidden");
    }

    if mode & FILE_ATTRIBUTE_INTEGRITY_STREAM == FILE_ATTRIBUTE_INTEGRITY_STREAM {
        v.push("Integrity Stream");
    }

    if mode & FILE_ATTRIBUTE_NORMAL == FILE_ATTRIBUTE_NORMAL {
        v.push("Normal");
    }

    if mode & FILE_ATTRIBUTE_NOT_CONTENT_INDEXED == FILE_ATTRIBUTE_NOT_CONTENT_INDEXED {
        v.push("Not indexed");
    }

    if mode & FILE_ATTRIBUTE_NO_SCRUB_DATA == FILE_ATTRIBUTE_NO_SCRUB_DATA {
        v.push("No scrub data");
    }

    if mode & FILE_ATTRIBUTE_OFFLINE == FILE_ATTRIBUTE_OFFLINE {
        v.push("Offline");
    }

    if mode & FILE_ATTRIBUTE_READONLY == FILE_ATTRIBUTE_READONLY {
        v.push("Readonly");
    }

    if mode & FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS == FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS {
        v.push("Recall on data access");
    }

    if mode & FILE_ATTRIBUTE_RECALL_ON_OPEN == FILE_ATTRIBUTE_RECALL_ON_OPEN {
        v.push("Recall on open");
    }

    if mode & FILE_ATTRIBUTE_REPARSE_POINT == FILE_ATTRIBUTE_REPARSE_POINT {
        v.push("Reparse point");
    }

    if mode & FILE_ATTRIBUTE_SPARSE_FILE == FILE_ATTRIBUTE_SPARSE_FILE {
        v.push("Sparse");
    }

    if mode & FILE_ATTRIBUTE_SYSTEM == FILE_ATTRIBUTE_SYSTEM {
        v.push("System");
    }

    if mode & FILE_ATTRIBUTE_TEMPORARY == FILE_ATTRIBUTE_TEMPORARY {
        v.push("Temporary");
    }

    if mode & FILE_ATTRIBUTE_VIRTUAL == FILE_ATTRIBUTE_VIRTUAL {
        v.push("Virtual");
    }

    v.join(", ")
}

const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x20;
const FILE_ATTRIBUTE_COMPRESSED: u32 = 0x800;
const FILE_ATTRIBUTE_DEVICE: u32 = 0x40;
const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
const FILE_ATTRIBUTE_ENCRYPTED: u32 = 0x4000;
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
const FILE_ATTRIBUTE_INTEGRITY_STREAM: u32 = 0x8000;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: u32 = 0x2000;
const FILE_ATTRIBUTE_NO_SCRUB_DATA: u32 = 0x20000;
const FILE_ATTRIBUTE_OFFLINE: u32 = 0x1000;
const FILE_ATTRIBUTE_READONLY: u32 = 0x1;
const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 0x400000;
const FILE_ATTRIBUTE_RECALL_ON_OPEN: u32 = 0x40000;
const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
const FILE_ATTRIBUTE_SPARSE_FILE: u32 = 0x200;
const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;
const FILE_ATTRIBUTE_TEMPORARY: u32 = 0x100;
const FILE_ATTRIBUTE_VIRTUAL: u32 = 0x10000;