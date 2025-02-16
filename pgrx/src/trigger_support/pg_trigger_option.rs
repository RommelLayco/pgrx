use crate::pg_sys;
use crate::trigger_support::{PgTriggerError, TriggerEvent};

/// The operation for which the trigger was fired
///
/// Maps from a `TEXT` of `INSERT`, `UPDATE`, `DELETE`, or `TRUNCATE`.
///
/// Can be calculated from a `pgrx_pg_sys::TriggerEvent`.
// Postgres constants: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h#L92
// Postgres defines: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h#L92
pub enum PgTriggerOperation {
    /// `INSERT`
    Insert,
    /// `UPDATE`
    Update,
    /// `DELETE`
    Delete,
    /// `TRUNCATE`
    Truncate,
}

impl TryFrom<TriggerEvent> for PgTriggerOperation {
    type Error = PgTriggerError;
    fn try_from(event: TriggerEvent) -> Result<Self, Self::Error> {
        match event.0 & pg_sys::TRIGGER_EVENT_OPMASK {
            pg_sys::TRIGGER_EVENT_INSERT => Ok(Self::Insert),
            pg_sys::TRIGGER_EVENT_DELETE => Ok(Self::Delete),
            pg_sys::TRIGGER_EVENT_UPDATE => Ok(Self::Update),
            pg_sys::TRIGGER_EVENT_TRUNCATE => Ok(Self::Truncate),
            v => Err(PgTriggerError::InvalidPgTriggerOperation(v)),
        }
    }
}

impl ToString for PgTriggerOperation {
    fn to_string(&self) -> String {
        match self {
            PgTriggerOperation::Insert => "INSERT",
            PgTriggerOperation::Update => "UPDATE",
            PgTriggerOperation::Delete => "DELETE",
            PgTriggerOperation::Truncate => "TRUNCATE",
        }
        .to_string()
    }
}
