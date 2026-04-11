use anyhow::{anyhow, Result};

use itertools::Either;
use rusqlite::{functions::FunctionFlags, params, OptionalExtension as _};

use crate::storage::{value::ArchivedStoreValue, Store, StoreValue};
use rkyv::rancor::{self, Error};

#[derive(Debug)]
pub struct SQLiteStore {
	connection: rusqlite::Connection,
}

impl SQLiteStore {
	pub fn new(path: &str) -> Self {
		let conn = rusqlite::Connection::open(path).expect("To Open SQLite DB");

		conn
			.execute_batch(include_str!("schema.sql"))
			.expect("To Create DB");

		conn
			.create_scalar_function(
				"cfs_arr_push",
				2,
				FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
				|ctx| {
					let arg1 = ctx
						.get_raw(0)
						.as_blob()
						.expect("first arg to cfs_arr_push should be a BLOB");

					let arg2 = ctx
						.get_raw(1)
						.as_blob()
						.expect("second arg to cfs_arr_push should be a BLOB");

					let arg1 = unsafe { rkyv::access_unchecked::<ArchivedStoreValue>(&arg1) };
					let arg1 = match arg1 {
						ArchivedStoreValue::Value(value) => Either::Left(std::iter::once(value)),
						ArchivedStoreValue::List(values) => Either::Right(values.into_iter()),
					};

					let arg2 = unsafe { rkyv::access_unchecked::<ArchivedStoreValue>(&arg2) };
					let arg2 = match arg2 {
						ArchivedStoreValue::Value(value) => Either::Left(std::iter::once(value)),
						ArchivedStoreValue::List(values) => Either::Right(values.into_iter()),
					};

					let new_list = arg1
						.chain(arg2)
						.map(|archived| {
							rkyv::deserialize::<String, Error>(archived).expect("to deserialize string")
						})
						.collect::<Vec<_>>();

					let blob = rkyv::to_bytes::<Error>(&StoreValue::List(new_list))
						.expect("to serialize list")
						.into_vec();

					return Ok(blob);
				},
			)
			.expect("to create db function");

		Self { connection: conn }
	}
}

impl Store for SQLiteStore {
	fn all(&self) -> Result<Vec<(String, super::StoreValue)>> {
		let mut query = self.connection.prepare("SELECT key,value from KV")?;

		let values = query
			.query_map([], |row| {
				let blob = row.get_ref(1)?.as_blob()?;
				let value = unsafe {
					rkyv::from_bytes_unchecked::<StoreValue, rancor::Error>(&blob)
						.expect("to deserialize data")
				};

				return Ok((row.get(0)?, value));
			})?
			.collect::<Result<Vec<_>, _>>()?;

		Ok(values)
	}

	fn get(&self, key: &str) -> Result<Option<super::StoreValue>> {
		let query = self
			.connection
			.query_row(
				"SELECT key,value from KV where key = ?1 LIMIT 1",
				[key],
				|row| {
					let blob = row.get_ref(1)?.as_blob()?;
					let value = unsafe {
						rkyv::from_bytes_unchecked::<StoreValue, rancor::Error>(&blob)
							.expect("to deserialize data")
					};

					return Ok(value);
				},
			)
			.optional()?;

		Ok(query)
	}

	fn set(&mut self, key: &str, value: StoreValue) -> Result<StoreValue> {
		let blob = rkyv::to_bytes::<Error>(&value)?;

		self.connection.execute(
			"INSERT INTO KV VALUES(NULL,?1,?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
			params![key, blob.as_slice()],
		)?;

		Ok(value)
	}

	fn list_push(&mut self, key: &str, value: StoreValue) -> Result<StoreValue> {
		let blob = rkyv::to_bytes::<Error>(&value)?;

		self.connection.execute(
			"INSERT INTO KV VALUES(NULL,?1,?2) ON CONFLICT(key) DO UPDATE SET value = cfs_arr_push(value,?2)",
			params![key, blob.as_slice()],
		)?;

		Ok(value)
	}

	fn list_pop(&mut self, key: &str) -> Result<Option<StoreValue>> {
		let Some(value) = self.get(key)? else {
			return Ok(None);
		};

		let StoreValue::List(mut list) = value else {
			return Err(anyhow!("value at '{}' is not a list", key));
		};

		let Some(ret) = list.pop() else {
			//TODO: Add Storevalue::Null for cases like this?
			return Ok(Some(StoreValue::List(vec![])));
		};

		self.set(key, StoreValue::List(list))?;

		Ok(Some(StoreValue::Value(ret)))
	}

	fn remove(&mut self, key: &str) -> Result<Option<StoreValue>> {
		let value = self.get(key)?;

		let Some(value) = value else {
			return Ok(None);
		};

		let query = self
			.connection
			.execute("DELETE FROM KV where key = ?1", [key])?;

		if query == 0 {
			panic!("Deleted 0 Rows when trying to delete Value from Store")
		}

		Ok(Some(value))
	}

	fn clear(&mut self) -> Result<usize> {
		Ok(self.connection.execute("DELETE from KV;", [])?)
	}
}
