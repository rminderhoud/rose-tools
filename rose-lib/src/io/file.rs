use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use failure::Error;
use io::{ReadRoseExt, WriteRoseExt};

pub trait RoseFile {
    /// Construct a new file
    ///
    /// # Example
    /// ```rust
    /// use roselib::files::ZMS;
    /// use roselib::io::RoseFile;
    ///
    /// let _ = ZMS::new();
    /// ```
    fn new() -> Self;

    /// Read data from a reader
    fn read<R: ReadRoseExt>(&mut self, reader: &mut R) -> Result<(), Error>;

    /// Write data to a writer
    fn write<W: WriteRoseExt>(&mut self, writer: &mut W) -> Result<(), Error>;

    /// Create new RoseFile from a `File`
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::fs::File;
    /// use roselib::files::ZMS;
    /// use roselib::io::RoseFile;
    ///
    /// let f = File::open("foo.zms").unwrap();
    /// let _ = ZMS::from_file(&f);
    /// ```
    fn from_file(file: &File) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut rf = Self::new();
        rf.read_from_file(file)?;
        Ok(rf)
    }

    /// Create new RoseFile from a `Path`
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::path::PathBuf;
    /// use roselib::files::ZMS;
    /// use roselib::io::RoseFile;
    ///
    /// let p = PathBuf::from("/path/to/my.zms");
    /// let _ = ZMS::from_path(&p);
    /// ```
    fn from_path(path: &Path) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let f = File::open(path)?;
        Self::from_file(&f)
    }

    /// Read data from a `File`
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::fs::File;
    /// use roselib::files::ZMS;
    /// use roselib::io::RoseFile;
    ///
    /// let f = File::create("foo.zms").unwrap();
    /// let mut zms = ZMS::new();
    /// let _ = zms.read_from_file(&f);
    /// ```
    ///
    fn read_from_file(&mut self, file: &File) -> Result<(), Error> {
        let mut reader = BufReader::new(file);
        self.read(&mut reader)?;
        Ok(())
    }

    /// Write data to a `File`
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::fs::File;
    /// use roselib::files::ZMS;
    /// use roselib::io::RoseFile;
    ///
    /// let f = File::create("foo.zms").unwrap();
    /// let mut zms = ZMS::new();
    /// let _ = zms.write_to_file(&f);
    /// ```
    fn write_to_file(&mut self, file: &File) -> Result<(), Error> {
        let mut writer = BufWriter::new(file);
        self.write(&mut writer)?;
        Ok(())
    }

    /// Read data to the file from `Path`
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::path::PathBuf;
    /// use roselib::files::ZMS;
    /// use roselib::io::RoseFile;
    ///
    /// let p = PathBuf::from("/path/to/my.zms");
    /// let mut zms = ZMS::new();
    /// zms.read_from_path(&p);
    fn read_from_path(&mut self, path: &Path) -> Result<(), Error> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        self.read(&mut reader)?;
        Ok(())
    }

    /// Write data to a file at `Path`
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::path::PathBuf;
    /// use roselib::files::ZMS;
    /// use roselib::io::RoseFile;
    ///
    /// let p = PathBuf::from("/path/to/my.zms");
    /// let mut zms = ZMS::new();
    /// let _  = zms.write_to_path(&p);
    fn write_to_path(&mut self, path: &Path) -> Result<(), Error> {
        let f = File::open(path)?;
        self.write_to_file(&f)?;
        Ok(())
    }
}
