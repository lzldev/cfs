use rkyv::util::AlignedVec;
use rusqlite::{
	functions::SqlFnOutput,
	types::{ToSqlOutput, ValueRef},
};

pub struct AlignedVecWrapper(pub AlignedVec);

impl SqlFnOutput for AlignedVecWrapper {
	fn to_sql(&self) -> rusqlite::Result<(ToSqlOutput<'_>, rusqlite::functions::SubType)> {
		Ok((
			ToSqlOutput::Borrowed(ValueRef::Blob(self.0.as_slice())),
			None,
		))
	}
}
