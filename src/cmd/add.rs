
use std::{fmt::write, fs::metadata, os::unix::{ffi::OsStrExt, fs::MetadataExt}, path::PathBuf};

use pathdiff::diff_paths;
use serde::{Deserialize, Serialize};
use clap::Parser;

use crate::{Exec, Invocation};

#[derive(Parser)]
pub struct AddCmd {
    paths: Vec<PathBuf>,
}

fn to_index_time(time: i64) -> git2::IndexTime {
    git2::IndexTime::new(
        time.try_into()
            .expect("current version of libgit2 does not support timestamps beyond year 2038"),
        0
    )
}

impl Exec for AddCmd {

    fn exec(&self, inv: &Invocation) -> anyhow::Result<()> {
        let repo = inv.repo()?;
        // let last_commit = repo.head()?.peel_to_commit();
        let mut index = repo.index()?;
        // let state_path = inv.meta_dir.join("state.json");
        // let mut state: State = match std::fs::File::open(&state_path) {
        //     Ok(reader) => serde_json::from_reader(reader)?,
        //     Err(error) if error.kind() == std::io::ErrorKind::NotFound => State::new(),
        //     Err(error) => return Err(error.into()),
        // };
        for path in &self.paths {
            let abs_path = std::path::absolute(path)?;
            let rel_path = diff_paths(&abs_path, &inv.root_path)
                .expect(&format!("path does not refer to a file inside {}", inv.root_path.display()));
            let oid = repo.blob_path(&abs_path)?;
            let metadata = std::fs::metadata(abs_path)?;
            index.add(&git2::IndexEntry {
                flags: 0,
                id: oid,
                path: rel_path.as_os_str().as_bytes().to_vec(),
                flags_extended: 0,
                gid: metadata.gid(),
                uid: metadata.uid(),
                mode: metadata.mode(),
                ino: metadata.ino().try_into().expect("inode number is too large to be stored in Git"),
                dev: metadata.dev().try_into().expect("device ID is too large to be stored in Git"),
                file_size: metadata.size().try_into().expect("file size is too large to be stored in Git"),
                ctime: to_index_time(metadata.ctime()),
                mtime: to_index_time(metadata.mtime()),
            })?;
            // state.files.push(FileInfo {
            //     path: rel_path,
            // });
        }
        index.write()?;
        // std::fs::create_dir_all(state_path.parent().unwrap())?;
        // let writer = std::fs::OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .truncate(true)
        //     .open(&state_path)?;
        // serde_json::to_writer_pretty(writer, &state)?;
        Ok(())
    }

}

#[derive(Serialize, Deserialize)]
struct State {
    version: u32,
    files: Vec<FileInfo>,
}

impl State {

    pub fn new() -> Self {
        Self {
            version: 0x000000000001,
            files: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FileInfo {
    path: PathBuf,
}

// fn read_byte_vec<R: std::io::Read>(reader: &mut R) -> std::io::Result<Vec<u8>> {
//     let n = reader.read_u32::<BigEndian>()?.try_into().expect("persistent string does not fit into memory");
//     let mut buf = Vec::with_capacity(n);
//     // SAFETY: We either fill the buffer completely or discard it in its entirety
//     unsafe { buf.set_len( n) }
//     reader.read_exact(&mut buf);
//     Ok(buf)
// }

// fn read_entries<R: std::io::Read>(reader: &mut R) -> BTreeMap<OsString, FileInfo> {
//     let out = Vec::new();
//     loop {
//         let path = read_byte_vec(reader)?;
//         let fileinfo = read_file_info(reader)?;
//     }
//     Ok(out)
// }
