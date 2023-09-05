use std::fmt;

use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum DbError {
    InternalServerError,
    NotFound,
}


pub fn db_internal_error<E>(_err: E) -> DbError
{
    DbError::InternalServerError
}

pub fn diesel_error(err: DieselError) -> DbError
{
    match err {
        DieselError::NotFound => DbError::NotFound,
        _ => DbError::InternalServerError,
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::NotFound => write!(f, "Not found"),
            DbError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}
