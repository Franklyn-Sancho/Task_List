use bytes::BytesMut;
use postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

use crate::interfaces::cli::task_list::{Priority, Status};

impl ToSql for Priority {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let value = match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };
        out.extend_from_slice(value.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for Priority {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn  std::error::Error + Sync + Send>> {
        let value = std::str::from_utf8(raw)?;
        
        match value {
            "Low" => Ok(Priority::Low),
            "Medium" => Ok(Priority::Medium),
            "High" => Ok(Priority::High),
            _ => Err("Invalid priority value".into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        <&str as FromSql>::accepts(ty)
    }
}


impl<'a> FromSql<'a> for Status {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let value = std::str::from_utf8(raw)?;
        
        match value {
            "Pending" => Ok(Status::Pendent),
            "Completed" => Ok(Status::Completed),
            _ => Err("Invalid status value".into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        <&str as FromSql>::accepts(ty)
    }
}

impl ToSql for Status {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let value = match self {
            Status::Pendent => "Pending",
            Status::Completed => "Completed",
        };
        out.extend_from_slice(value.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}