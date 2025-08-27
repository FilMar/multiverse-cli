//! Database struct generation macros for relations
//! These macros create database handler structs instead of loose functions

/// Macro to generate a database handler struct for a relation
#[macro_export]
macro_rules! define_relation_db_struct {
    (
        $struct_name:ident,
        table: $table_name:literal,
        key_fields: {
            $key1:ident: $key_type1:ty,
            $key2:ident: $key_type2:ty
        },
        fields: {
            $($field_name:ident: $field_type:ty),+ $(,)?
        },
        create_sql: $create_sql:literal
    ) => {
        pub struct $struct_name;

        impl $struct_name {
            /// Initialize the relation table
            pub fn init_table(conn: &rusqlite::Connection) -> anyhow::Result<()> {
                conn.execute($create_sql, [])
                    .map_err(|e| anyhow::anyhow!("Failed to create {} table: {}", $table_name, e))?;
                Ok(())
            }

            /// Insert a new relation
            pub fn insert(
                conn: &rusqlite::Connection,
                $key1: $key_type1,
                $key2: &str,
                $($field_name: &str),+
            ) -> anyhow::Result<()> {
                let mut columns = vec![stringify!($key1).to_string(), stringify!($key2).to_string()];
                $(
                    columns.push(stringify!($field_name).to_string());
                )*
                columns.push("created_at".to_string());
                
                let columns_clause = columns.join(", ");
                let values_placeholders = (1..=columns.len()).map(|i| format!("?{}", i)).collect::<Vec<_>>().join(", ");
                
                let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    $table_name,
                    columns_clause,
                    values_placeholders
                );
                
                conn.execute(&sql, rusqlite::params![
                    $key1,
                    $key2.to_string(),
                    $($field_name.to_string()),+,
                    chrono::Utc::now().to_rfc3339()
                ])?;
                
                Ok(())
            }

            /// Update an existing relation
            pub fn update(
                conn: &rusqlite::Connection,
                $key1: $key_type1,
                $key2: &str,
                $($field_name: &str),+
            ) -> anyhow::Result<()> {
                let mut field_assignments = Vec::new();
                $(
                    field_assignments.push(format!("{} = ?", stringify!($field_name)));
                )*
                let set_clause = field_assignments.join(", ");
                
                let sql = format!(
                    "UPDATE {} SET {} WHERE {} = ? AND {} = ?",
                    $table_name,
                    set_clause,
                    stringify!($key1),
                    stringify!($key2)
                );
                
                conn.execute(&sql, rusqlite::params![
                    $($field_name.to_string()),+,
                    $key1,
                    $key2.to_string()
                ])?;
                
                Ok(())
            }

            /// Delete a relation
            pub fn delete(
                conn: &rusqlite::Connection,
                $key1: $key_type1,
                $key2: &str
            ) -> anyhow::Result<()> {
                let sql = format!(
                    "DELETE FROM {} WHERE {} = ?1 AND {} = ?2",
                    $table_name,
                    stringify!($key1),
                    stringify!($key2)
                );
                
                conn.execute(&sql, rusqlite::params![$key1, $key2.to_string()])?;
                Ok(())
            }

            /// Get all relations by first key
            pub fn get_by_first_key(
                conn: &rusqlite::Connection,
                key_value: $key_type1
            ) -> anyhow::Result<Vec<(String, String, String, String)>> {
                let mut select_columns = vec![stringify!($key2).to_string()];
                $(
                    select_columns.push(stringify!($field_name).to_string());
                )*
                let select_clause = select_columns.join(", ");
                
                let sql = format!(
                    "SELECT {} FROM {} WHERE {} = ?1",
                    select_clause,
                    $table_name,
                    stringify!($key1)
                );
                
                let mut stmt = conn.prepare(&sql)?;
                let rows = stmt.query_map([key_value], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?
                    ))
                })?;
                
                let mut results = Vec::new();
                for row in rows {
                    results.push(row?);
                }
                
                Ok(results)
            }
        }
    };
}
