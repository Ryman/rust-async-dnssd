use std::io;

use raw;

/// A successful record registration
///
/// Releases the record when dropped (unless it is a
/// [`Registration::get_default_txt_record`]
/// (struct.Registration.html#method.get_default_txt_record) or a 
/// [`Register::get_default_txt_record`]
/// (struct.Register.html#method.get_default_txt_record))
///
/// Also keeps the underlying [`Registration`](struct.Registration.html)
/// or [`Connection`](struct.Connection.html) alive.
pub struct Record(raw::DNSRecord);

impl Record {
	/// Type of the record
	pub fn rr_type(&self) -> u16 {
		self.0.rr_type()
	}

	/// Update recor
	///
	/// Cannot change type or class of record.
	///
	/// See [`DNSServiceUpdateRecord`](https://developer.apple.com/documentation/dnssd/1804739-dnsserviceupdaterecord).
	pub fn update_raw_record(
		&self,
		rdata: &[u8],
		ttl: u32
	) -> io::Result<()> {
		self.0.update_record(
			0, /* no flags */
			rdata,
			ttl
		)?;
		Ok(())
	}

	/// Keep record alive for as long as the underlying
	/// [`Registration`](struct.Registration.html) or
	/// [`Connection`](struct.Connection.html) lives
	pub fn keep(self) {
		self.0.keep()
	}
}

pub fn new_record(r: raw::DNSRecord) -> Record {
	Record(r)
}
