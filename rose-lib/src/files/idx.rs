//! ROSE Online Virtual File Systems
//!
//! ROSE Online uses a virtual file system to pack assets and ship with the
//! client. Assets are stored in binary blobs (.vfs) with an index (.idx)
//! containing the metadata for each asset file in the virtual files system.
//! Each file in the virtual file system maintains a 'filepath' which preserves
//! the directory hierarchy when unpacking the VFS.
//!
//! To interact with the virtual file system, start with the `VfsIndex` as it
//! contains all the information. It can be constructed manually in tandem
//! with a `.vfs` file or it can be loaded from disk.
//!
//! # Examples
//!
//! Load and interact with an existing index from a `.idx` file:
//!
//! ```rust,no_run
//! use std::path::Path;
//! use roselib::files::IDX;
//! use roselib::io::RoseFile;
//!
//! let idx = IDX::from_path(Path::new("/path/to/index.idx")).unwrap();
//!
//! for vfs in idx.file_systems {
//!     for vfs_file in vfs.files {
//!         println!("File: {}", vfs_file.filepath.to_str().unwrap_or(""));
//!     }
//! }
//! ```
use std::io::SeekFrom;
use std::path::PathBuf;

use failure::Error;

use crate::io::{PathRoseExt, ReadRoseExt, RoseFile, WriteRoseExt};

/// Virtual file system index file
pub type IDX = VfsIndex;

/// Virtual file system index
///
/// An index of the virtual file systems, usually suffixed with `.idx`.
/// The index does not contain any actual asset data, only meta data about
/// the file systems. Each file system in the index usually maps to a single
/// `.vfs` file on disk.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VfsIndex {
    pub base_version: i32,
    pub current_version: i32,
    pub file_systems: Vec<VfsMetadata>,
}

impl RoseFile for VfsIndex {
    fn new() -> VfsIndex {
        Self::default()
    }

    /// Load a `VfsIndex` from a reader
    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error> {
        self.base_version = reader.read_i32()?;
        self.current_version = reader.read_i32()?;

        let vfs_count = reader.read_i32()?;
        for i in 0..vfs_count {
            let mut vfs = VfsMetadata::new();
            vfs.filename = PathBuf::from(reader.read_string_u16()?);

            let offset = reader.read_i32()?;
            let next_filesystem = reader.seek(SeekFrom::Current(0))?; // seek(0) returns current position
            let _ = reader.seek(SeekFrom::Start(offset as u64))?;

            let file_count = reader.read_i32()?;
            let _delete_count = reader.read_i32()?;
            let _start_offset = reader.read_i32()?;

            for _ in 0..file_count {
                let mut vfs_file = VfsFileMetadata::new();
                vfs_file.filepath = PathBuf::from_rose_path(&reader.read_string_u16()?);
                vfs_file.offset = reader.read_i32()?;
                vfs_file.size = reader.read_i32()?;
                vfs_file.block_size = reader.read_i32()?;
                vfs_file.is_deleted = reader.read_bool()?;
                vfs_file.is_compressed = reader.read_bool()?;
                vfs_file.is_encrypted = reader.read_bool()?;
                vfs_file.version = reader.read_i32()?;
                vfs_file.checksum = reader.read_i32()?;

                vfs.files.push(vfs_file);
            }

            self.file_systems.push(vfs);
            if i < vfs_count - 1 {
                let _ = reader.seek(SeekFrom::Start(next_filesystem as u64));
            }
        }
        Ok(())
    }

    /// Save a `VfsIndex` to a writer
    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error> {
        writer.write_i32(self.base_version)?;
        writer.write_i32(self.current_version)?;
        writer.write_i32(self.file_systems.len() as i32)?;

        let mut file_system_offsets: Vec<u64> = vec![];

        for i in 0..self.file_systems.len() {
            let fname = &self.file_systems[i].filename.to_str().unwrap_or("");
            writer.write_string_u16(fname)?;

            file_system_offsets.push(writer.seek(SeekFrom::Current(0))?);
            writer.write_i32(0)?; // Reserve to be written later
        }

        for (i, vfs) in self.file_systems.iter().enumerate() {
            let file_offset = writer.seek(SeekFrom::Current(0))?;

            // Add data offset to header section
            writer.seek(SeekFrom::Start(file_system_offsets[i]))?;
            writer.write_i32(file_offset as i32)?;
            writer.seek(SeekFrom::Start(file_offset))?;

            let mut deleted_count: i32 = 0;
            for file in &vfs.files {
                if file.is_deleted {
                    deleted_count += 1;
                }
            }

            writer.write_i32(vfs.files.len() as i32)?;
            writer.write_i32(deleted_count)?;
            writer.write_i32(vfs.files[0].offset)?;

            for file in &vfs.files {
                let fname = &file.filepath.to_str().unwrap_or("");
                writer.write_string_u16(fname)?;
                writer.write_i32(file.offset)?;
                writer.write_i32(file.size)?;
                writer.write_i32(file.block_size)?;
                writer.write_bool(file.is_deleted)?;
                writer.write_bool(file.is_compressed)?;
                writer.write_bool(file.is_encrypted)?;
                writer.write_i32(file.version)?;
                writer.write_i32(file.checksum)?;
            }
        }
        Ok(())
    }
}

/// Virtual file system
///
/// Contains the metadata for a single file system.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VfsMetadata {
    pub filename: PathBuf,
    pub files: Vec<VfsFileMetadata>,
}

impl VfsMetadata {
    /// Construct an empty virtual file system
    pub fn new() -> VfsMetadata {
        Self::default()
    }
}

/// Virtual file system file entry
///
/// Contains the metadata for a single file in the file system
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VfsFileMetadata {
    pub filepath: PathBuf,
    pub offset: i32,
    pub size: i32,
    pub block_size: i32,
    pub is_deleted: bool,
    pub is_compressed: bool,
    pub is_encrypted: bool,
    pub version: i32,
    pub checksum: i32,
}

impl VfsFileMetadata {
    /// Construct an empty virtual file system file
    pub fn new() -> VfsFileMetadata {
        Self::default()
    }
}
