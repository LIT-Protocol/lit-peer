mod consts;
mod error;
mod models;
mod queries;

use std::collections::BTreeMap;
use std::fmt::Debug;

pub use consts::*;
pub use error::*;
pub use models::*;
pub use osquery_rs::OSQuery;
pub use queries::*;

pub fn get_default_os_query() -> OSQuery {
    OSQuery::new().set_socket(consts::OSQUERY_SOCKET)
}

/// Execute a query against the os_query instance and return the results
pub fn execute_query<T>(os_query: &OSQuery, query: String) -> Result<Vec<T>>
where
    T: Debug + for<'a> TryFrom<&'a BTreeMap<String, String>, Error = String>,
{
    let res = os_query
        .query(query)
        .map_err(|e| unexpected_err(e, Some("Unable to query osquery".into())))?;
    let mut rows = Vec::new();
    if let Some(status) = &res.status {
        if let Some(code) = status.code {
            if code != 0 {
                return Err(io_err_code("Invalid input", error::EC::InvalidInput, None));
            }

            if let Some(response) = &res.response {
                for row in response {
                    if row.is_empty() {
                        continue;
                    }
                    let proc = T::try_from(row)
                        .map_err(|e| unexpected_err(e, Some("Unable to parse row".into())))?;
                    rows.push(proc);
                }
            }
        }
    }
    Ok(rows)
}
