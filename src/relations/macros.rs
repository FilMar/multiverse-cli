//! Generic relation macro system inspired by entity macros

/// Generic macro to define any relation between two entity types
#[macro_export]
macro_rules! define_relation {
    (
        $relation_name:ident,
        $from_entity:ident -> $to_entity:ident,
        table: $table_name:literal,
        from_table: $from_table:literal,
        to_table: $to_table:literal,
        fields: {
            $($field_name:ident: $field_type:ty),* $(,)?
        }
    ) => {
        use rusqlite::Connection;
        use anyhow::Result;

        #[derive(Debug, Clone)]
        pub struct $relation_name {
            pub from_id: String,
            pub to_id: String,
            $(pub $field_name: $field_type),*,
        }

        impl $relation_name {
            /// Create new relation instance
            pub fn new(from_id: String, to_id: String, $($field_name: $field_type),*) -> Self {
                Self {
                    from_id,
                    to_id,
                    $($field_name),*,
                }
            }

            /// Create this relation in the database
            pub fn create(&self) -> Result<()> {
                let db_path = crate::world::WorldConfig::get_database_path()?;
                let conn = crate::database::get_connection(&db_path)?;

                Relations::create_relation(&conn, &self.from_id, &self.to_id, $(&self.$field_name),*)?;

                // Success message will be handled by the caller

                Ok(())
            }

            /// Update existing relation in database
            pub fn update(&self) -> Result<()> {
                let db_path = crate::world::WorldConfig::get_database_path()?;
                let conn = crate::database::get_connection(&db_path)?;

                Relations::update_relation(&conn, &self.from_id, &self.to_id, $(&self.$field_name),*)?;

                Ok(())
            }

            /// Check if relation exists
            pub fn exists(&self) -> Result<bool> {
                let db_path = crate::world::WorldConfig::get_database_path()?;
                let conn = crate::database::get_connection(&db_path)?;

                Relations::relation_exists(&conn, &self.from_id, &self.to_id)
            }

            /// Create or update relation (upsert)
            pub fn upsert(&self) -> Result<bool> {
                if self.exists()? {
                    self.update()?;
                    Ok(false) // Updated existing
                } else {
                    self.create()?;
                    Ok(true) // Created new
                }
            }

            /// Delete this relation from the database
            pub fn delete(&self) -> Result<()> {
                let db_path = crate::world::WorldConfig::get_database_path()?;
                let conn = crate::database::get_connection(&db_path)?;

                Relations::delete_relation(&conn, &self.from_id, &self.to_id)?;

                println!("🗑️ Deleted relation: {} -> {}", self.from_id, self.to_id);

                Ok(())
            }

            /// List all relations for a from_entity
            pub fn list_for_entity(from_id: &str) -> Result<Vec<Self>> {
                let db_path = crate::world::WorldConfig::get_database_path()?;
                let conn = crate::database::get_connection(&db_path)?;
                
                let field_list = concat!("to_id", $(", ", stringify!($field_name))*);
                let sql = format!("SELECT {} FROM {} WHERE from_id = ?", field_list, $table_name);
                let mut stmt = conn.prepare(&sql)?;

                let rows = stmt.query_map([from_id], |row| {
                    Ok(Self::new(
                        from_id.to_string(),
                        row.get::<_, String>("to_id")?,
                        $(row.get::<_, String>(stringify!($field_name))?),*
                    ))
                })?;

                let mut results = Vec::new();
                for row in rows {
                    results.push(row?);
                }
                
                Ok(results)
            }
        }


        pub struct Relations;

        impl Relations {
            pub fn init_table(conn: &Connection) -> Result<()> {
                let field_defs = concat!($(stringify!($field_name), " TEXT, ",)*);
                let sql = format!(
                    "CREATE TABLE IF NOT EXISTS {} (
                        from_id TEXT NOT NULL,
                        to_id TEXT NOT NULL,
                        {}
                        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                        PRIMARY KEY (from_id, to_id),
                        FOREIGN KEY (from_id) REFERENCES {} (id),
                        FOREIGN KEY (to_id) REFERENCES {} (id)
                    )",
                    $table_name,
                    field_defs,
                    $from_table,
                    $to_table
                );
                
                conn.execute(&sql, [])?;
                Ok(())
            }

            pub fn create_relation(
                conn: &Connection,
                from_id: &str,
                to_id: &str,
                $($field_name: &$field_type),*
            ) -> Result<()> {
                let mut field_names = vec!["from_id", "to_id"];
                let mut placeholders = vec!["?", "?"];
                let mut values: Vec<&dyn rusqlite::ToSql> = vec![&from_id, &to_id];

                $(
                    field_names.push(stringify!($field_name));
                    placeholders.push("?");
                    values.push($field_name);
                )*

                field_names.push("created_at");
                placeholders.push("CURRENT_TIMESTAMP");

                let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    $table_name,
                    field_names.join(", "),
                    placeholders.join(", ")
                );

                conn.execute(&sql, &values[..])?;
                Ok(())
            }

            pub fn delete_relation(conn: &Connection, from_id: &str, to_id: &str) -> Result<()> {
                let sql = format!("DELETE FROM {} WHERE from_id = ? AND to_id = ?", $table_name);
                conn.execute(&sql, [from_id, to_id])?;
                Ok(())
            }

            pub fn update_relation(
                conn: &Connection,
                from_id: &str,
                to_id: &str,
                $($field_name: &$field_type),*
            ) -> Result<()> {
                let mut update_fields = Vec::new();
                let mut values: Vec<&dyn rusqlite::ToSql> = Vec::new();
                
                $(
                    update_fields.push(format!("{} = ?", stringify!($field_name)));
                    values.push($field_name);
                )*
                
                values.push(&from_id);
                values.push(&to_id);
                
                let sql = format!(
                    "UPDATE {} SET {} WHERE from_id = ? AND to_id = ?",
                    $table_name,
                    update_fields.join(", ")
                );
                
                let rows_affected = conn.execute(&sql, &values[..])?;
                
                if rows_affected == 0 {
                    return Err(anyhow::anyhow!("No relation found to update between {} and {}", from_id, to_id));
                }

                Ok(())
            }

            pub fn relation_exists(conn: &Connection, from_id: &str, to_id: &str) -> Result<bool> {
                let check_sql = format!("SELECT COUNT(*) FROM {} WHERE from_id = ? AND to_id = ?", $table_name);
                let mut stmt = conn.prepare(&check_sql)?;
                let count: i64 = stmt.query_row([from_id, to_id], |row| row.get(0))?;
                
                Ok(count > 0)
            }
        }
    };
}