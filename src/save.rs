use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use libflate::gzip::{Decoder, Encoder};
use std::cmp::min;
use std::collections::HashSet;
use std::convert::TryInto;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result};

/// A save file.
#[derive(Debug)]
pub struct ArchiveReader<R: Read> {
	/// The GZip decoder decoding the file.
	decoder: Decoder<R>,

	/// The number of bytes remaining in the current member’s file content.
	member_bytes_left: usize,
}

impl<R: Read> ArchiveReader<R> {
	/// Returns the next member in the file.
	///
	/// If there are no more members in the archive, `None` is returned.
	pub fn next(&mut self) -> Result<Option<Member<'_, R>>> {
		// If the current member isn’t finished, finish it.
		if self.member_bytes_left != 0 {
			self.finish_current_member()?;
		}

		// Read the name, which comprises a little-endian 32-bit length followed by that many
		// little-endian UTF-16 code units.
		let name_length = match self.read_u32()? {
			// Cast is sound because usize ≥ 32.
			Some(n) => n as usize,
			None => return Ok(None),
		};
		let mut name_buf: Vec<u16> = Vec::new();
		name_buf.resize(name_length, 0_u16);
		self.decoder.read_u16_into::<LittleEndian>(&mut name_buf)?;
		let name = match String::from_utf16(&name_buf) {
			Ok(n) => n,
			Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
		};
		drop(name_buf);

		// Read the file length, a little-endian 32-bit length. Cast is sound because usize ≥ 32.
		let size = self.decoder.read_u32::<LittleEndian>()? as usize;
		self.member_bytes_left = size;

		Ok(Some(Member {
			name,
			size,
			container: self,
		}))
	}

	/// Reads to the end of the current member’s file content.
	fn finish_current_member(&mut self) -> Result<()> {
		let mut buffer = [0_u8; 4096];
		while self.member_bytes_left != 0 {
			let to_read = min(buffer.len(), self.member_bytes_left);
			let bytes_read = self.decoder.read(&mut buffer[..to_read])?;
			self.member_bytes_left -= bytes_read;
		}
		Ok(())
	}

	/// Reads a single `u32`.
	///
	/// If the file is at EOF, `None` is returned. If four bytes can be read successfully, their
	/// value as a little-endian `u32` is returned. If at least one byte, but nor four, can be
	/// read, `std::io::ErrorKind::UnexpectedEof` is returned.
	fn read_u32(&mut self) -> Result<Option<u32>> {
		let mut buffer = [0_u8; 4];
		let mut buffer_filled = 0;
		while buffer_filled != 4 {
			let bytes_read = self.decoder.read(&mut buffer[buffer_filled..])?;
			if bytes_read == 0 {
				// EOF
				return if buffer_filled == 0 {
					Ok(None)
				} else {
					Err(std::io::Error::new(
						std::io::ErrorKind::UnexpectedEof,
						"EOF within archive member",
					))
				};
			}
			buffer_filled += bytes_read;
		}
		Ok(Some(u32::from_le_bytes(buffer)))
	}
}

/// A single member of a save file.
///
/// The `R` generic parameter indicates the type of the reader being used to read the save file.
/// The `'file` generic parameter indicates the lifetime of the save file.
#[derive(Debug)]
pub struct Member<'file, R: Read> {
	/// The member’s filename.
	name: String,

	/// The size of the member, in bytes.
	size: usize,

	/// The containing save file.
	container: &'file mut ArchiveReader<R>,
}

impl<'member, R: Read> Member<'member, R> {
	/// Returns the member’s filename.
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Returns the size of the member, in bytes.
	pub fn size(&self) -> usize {
		self.size
	}
}

impl<'member, R: Read> Read for Member<'member, R> {
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
		let to_read = min(buffer.len(), self.container.member_bytes_left);
		// libflate::gzip::Decoder does not like being called with a zero-length buffer (it
		// interprets the zero return value from the underlying stream as an EOF).
		// <https://github.com/sile/libflate/issues/61>
		let bytes_read = self.container.decoder.read(&mut buffer[..to_read])?;
		self.container.member_bytes_left -= bytes_read;
		Ok(bytes_read)
	}
}

/// Opens a save file for reading.
fn open_read(filename: &OsStr) -> Result<ArchiveReader<BufReader<File>>> {
	Ok(ArchiveReader {
		decoder: Decoder::new(BufReader::new(File::open(filename)?))?,
		member_bytes_left: 0,
	})
}

/// Lists the contents of a save file.
pub fn list(filename: &OsStr) -> Result<()> {
	let mut reader = open_read(filename)?;
	while let Some(member) = reader.next()? {
		println!("{}\t{}", member.name(), member.size());
	}
	Ok(())
}

/// Packs a save file.
pub fn pack(filename: &OsStr, members: &[&str]) -> Result<()> {
	let mut writer = Encoder::new(BufWriter::new(File::create(filename)?))?;
	for member in members {
		// Write the name, in little-endian UTF-16, preceded by its length in code units as a
		// little-endian u32.
		let name: Vec<u16> = member.encode_utf16().collect();
		let name_len: u32 = name
			.len()
			.try_into()
			.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Member name too long"))?;
		writer.write_u32::<LittleEndian>(name_len)?;
		name.iter()
			.try_for_each(|i| writer.write_u16::<LittleEndian>(*i))?;
		drop(name);

		// Write the file body, preceded by its length in bytes as a little-endian u32.
		let reader = File::open(member)?;
		let file_size = reader.metadata()?.len();
		let file_size: u32 = file_size
			.try_into()
			.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Member too large"))?;
		let mut reader = BufReader::new(reader.take(file_size.into()));
		writer.write_u32::<LittleEndian>(file_size)?;
		std::io::copy(&mut reader, &mut writer)?;
	}
	let writer = writer.finish().into_result()?;
	let writer = writer.into_inner()?;
	writer.sync_all()?;
	Ok(())
}

/// Unpacks a save file.
///
/// If the `members` set is nonempty on entry, only the named members are unpacked, and, on return,
/// it contains the subset of its original set that were not found in the archive. If the `members`
/// set is empty, all members are unpacked.
pub fn unpack(filename: &OsStr, members: &mut HashSet<&str>) -> Result<()> {
	let extract_all = members.is_empty();
	let mut reader = open_read(filename)?;
	while let Some(mut member) = reader.next()? {
		if extract_all || members.remove(member.name()) {
			let mut writer = BufWriter::new(File::create(member.name())?);
			std::io::copy(&mut member, &mut writer)?;
			writer.into_inner()?.sync_all()?;
		}
	}
	Ok(())
}
