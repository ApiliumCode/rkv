// Copyright 2018-2019 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use std::path::PathBuf;

use bincode;
use thiserror::Error;
use lmdb;

use crate::value::Type;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("unknown type tag: {0}")]
    UnknownType(u8),

    #[error("unexpected type tag: expected {expected}, got {actual}")]
    UnexpectedType {
        expected: Type,
        actual: Type,
    },

    #[error("empty data; expected tag")]
    Empty,

    #[error("invalid value for type {value_type}: {err}")]
    DecodingError {
        value_type: Type,
        err: Box<bincode::ErrorKind>,
    },

    #[error("couldn't encode value: {0}")]
    EncodingError(Box<bincode::ErrorKind>),

    #[error("invalid uuid bytes")]
    InvalidUuid,
}

impl From<Box<bincode::ErrorKind>> for DataError {
    fn from(e: Box<bincode::ErrorKind>) -> DataError {
        DataError::EncodingError(e)
    }
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("I/O error: {0:?}")]
    IoError(::std::io::Error),

    #[error("directory does not exist or not a directory: {0:?}")]
    DirectoryDoesNotExistError(PathBuf),

    #[error("data error: {0:?}")]
    DataError(DataError),

    #[error("lmdb error: {0}")]
    LmdbError(lmdb::Error),

    #[error("read transaction already exists in thread {0:?}")]
    ReadTransactionAlreadyExists(::std::thread::ThreadId),

    #[error("attempted to open DB during transaction in thread {0:?}")]
    OpenAttemptedDuringTransaction(::std::thread::ThreadId),
}

impl StoreError {
    pub fn open_during_transaction() -> StoreError {
        StoreError::OpenAttemptedDuringTransaction(::std::thread::current().id())
    }
}

impl From<lmdb::Error> for StoreError {
    fn from(e: lmdb::Error) -> StoreError {
        match e {
            lmdb::Error::BadRslot => StoreError::ReadTransactionAlreadyExists(::std::thread::current().id()),
            e => StoreError::LmdbError(e),
        }
    }
}

impl From<DataError> for StoreError {
    fn from(e: DataError) -> StoreError {
        StoreError::DataError(e)
    }
}

impl From<::std::io::Error> for StoreError {
    fn from(e: ::std::io::Error) -> StoreError {
        StoreError::IoError(e)
    }
}

#[derive(Debug, Error)]
pub enum MigrateError {
    #[error("database not found: {0:?}")]
    DatabaseNotFound(String),

    #[error("{0}")]
    FromString(String),

    #[error("couldn't determine bit depth")]
    IndeterminateBitDepth,

    #[error("I/O error: {0:?}")]
    IoError(::std::io::Error),

    #[error("invalid DatabaseFlags bits")]
    InvalidDatabaseBits,

    #[error("invalid data version")]
    InvalidDataVersion,

    #[error("invalid magic number")]
    InvalidMagicNum,

    #[error("invalid NodeFlags bits")]
    InvalidNodeBits,

    #[error("invalid PageFlags bits")]
    InvalidPageBits,

    #[error("invalid page number")]
    InvalidPageNum,

    #[error("lmdb error: {0}")]
    LmdbError(lmdb::Error),

    #[error("string conversion error")]
    StringConversionError,

    #[error("TryFromInt error: {0:?}")]
    TryFromIntError(::std::num::TryFromIntError),

    #[error("unexpected Page variant")]
    UnexpectedPageVariant,

    #[error("unexpected PageHeader variant")]
    UnexpectedPageHeaderVariant,

    #[error("unsupported PageHeader variant")]
    UnsupportedPageHeaderVariant,

    #[error("UTF8 error: {0:?}")]
    Utf8Error(::std::str::Utf8Error),
}

impl From<::std::io::Error> for MigrateError {
    fn from(e: ::std::io::Error) -> MigrateError {
        MigrateError::IoError(e)
    }
}

impl From<::std::str::Utf8Error> for MigrateError {
    fn from(e: ::std::str::Utf8Error) -> MigrateError {
        MigrateError::Utf8Error(e)
    }
}

impl From<::std::num::TryFromIntError> for MigrateError {
    fn from(e: ::std::num::TryFromIntError) -> MigrateError {
        MigrateError::TryFromIntError(e)
    }
}

impl From<&str> for MigrateError {
    fn from(e: &str) -> MigrateError {
        MigrateError::FromString(e.to_string())
    }
}

impl From<String> for MigrateError {
    fn from(e: String) -> MigrateError {
        MigrateError::FromString(e)
    }
}

impl From<lmdb::Error> for MigrateError {
    fn from(e: lmdb::Error) -> MigrateError {
        match e {
            e => MigrateError::LmdbError(e),
        }
    }
}
