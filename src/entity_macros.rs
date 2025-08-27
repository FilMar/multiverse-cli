//! New Modular Entity Macro System
//! Clean, composable macros without code duplication

/// Macro 1: Generate basic entity struct + status enum
#[macro_export]
macro_rules! define_entity_struct {
    (
        $entity:ident,
        $status:ident,
        key_fields: { $($key_field:ident: $key_type:ty),+ },
        fields: { $($field:ident: $field_type:ty),* },
        status_variants: [ $($variant:ident),+ ]
    ) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $entity {
            pub id: i32,  // Technical ID for relations
            $(pub $key_field: $key_type),+,  // Logical key fields
            $(pub $field: $field_type,)*
            #[serde(default)]
            pub metadata: std::collections::HashMap<String, serde_json::Value>,
            pub created_at: chrono::DateTime<chrono::Utc>,
            pub status: $status,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum $status {
            $($variant),+
        }

        impl Default for $status {
            fn default() -> Self {
                $crate::define_entity_struct!(@first_variant $($variant),+)
            }
        }
    };
    
    (@first_variant $first:ident, $($rest:ident),*) => {
        Self::$first
    };
    
    (@first_variant $only:ident) => {
        Self::$only
    };
}

/// Macro 2: Generate database handler struct
#[macro_export]
macro_rules! define_entity_database {
    (
        $db_struct:ident,
        $entity:ident,
        table: $table:literal,
        key_fields: { $($key_field:ident: $key_type:ty),+ },
        fields: { $($field:ident: $field_type:ty),* },
        create_sql: $sql:literal
    ) => {
        pub struct $db_struct;

        impl $db_struct {
            /// Initialize table
            pub fn init_table(conn: &rusqlite::Connection) -> anyhow::Result<()> {
                conn.execute($sql, [])?;
                Ok(())
            }

            /// Insert entity and return assigned ID
            pub fn insert(conn: &rusqlite::Connection, entity: &$entity) -> anyhow::Result<i32> {
                let mut columns = Vec::new();
                $(columns.push(stringify!($key_field));)+
                $(columns.push(stringify!($field));)*
                columns.extend(&["metadata", "created_at", "status"]);

                let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("?{}", i)).collect();
                let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({}) RETURNING id",
                    $table,
                    columns.join(", "),
                    placeholders.join(", ")
                );

                let metadata_json = serde_json::to_string(&entity.metadata)?;
                let status_str = serde_json::to_string(&entity.status)?.trim_matches('"').to_string();

                let id = conn.query_row(&sql, rusqlite::params![
                    $(entity.$key_field,)+
                    $(entity.$field,)*
                    metadata_json,
                    entity.created_at.to_rfc3339(),
                    status_str
                ], |row| row.get::<_, i32>(0))?;

                Ok(id)
            }

            /// Get by logical key(s)
            pub fn get_by_key(conn: &rusqlite::Connection, $($key_field: &$key_type),+) -> anyhow::Result<Option<$entity>> {
                let mut columns = vec!["id".to_string()];
                $(columns.push(stringify!($key_field).to_string());)+
                $(columns.push(stringify!($field).to_string());)*
                columns.extend(vec!["metadata".to_string(), "created_at".to_string(), "status".to_string()]);

                let mut where_clauses = Vec::new();
                $(where_clauses.push(format!("{} = ?", stringify!($key_field)));)+

                let sql = format!(
                    "SELECT {} FROM {} WHERE {}",
                    columns.join(", "),
                    $table,
                    where_clauses.join(" AND ")
                );

                let mut stmt = conn.prepare(&sql)?;
                let mut rows = stmt.query_map(rusqlite::params![$($key_field,)+], |row| {
                    Self::row_to_entity(row)
                })?;

                match rows.next() {
                    Some(entity) => Ok(Some(entity?)),
                    None => Ok(None),
                }
            }

            /// Get ID by logical key(s) - for relations
            pub fn get_id_by_key(conn: &rusqlite::Connection, $($key_field: &$key_type),+) -> anyhow::Result<Option<i32>> {
                let mut where_clauses = Vec::new();
                $(where_clauses.push(format!("{} = ?", stringify!($key_field)));)+

                let sql = format!(
                    "SELECT id FROM {} WHERE {}",
                    $table,
                    where_clauses.join(" AND ")
                );

                let mut stmt = conn.prepare(&sql)?;
                let mut rows = stmt.query_map(rusqlite::params![$($key_field,)+], |row| {
                    Ok(row.get::<_, i32>(0)?)
                })?;

                match rows.next() {
                    Some(id) => Ok(Some(id?)),
                    None => Ok(None),
                }
            }

            /// List all entities
            pub fn list(conn: &rusqlite::Connection) -> anyhow::Result<Vec<$entity>> {
                let mut columns = vec!["id".to_string()];
                $(columns.push(stringify!($key_field).to_string());)+
                $(columns.push(stringify!($field).to_string());)*
                columns.extend(vec!["metadata".to_string(), "created_at".to_string(), "status".to_string()]);

                let sql = format!(
                    "SELECT {} FROM {} ORDER BY created_at DESC",
                    columns.join(", "),
                    $table
                );

                let mut stmt = conn.prepare(&sql)?;
                let entity_iter = stmt.query_map([], |row| Self::row_to_entity(row))?;

                let mut entities = Vec::new();
                for entity in entity_iter {
                    entities.push(entity?);
                }
                Ok(entities)
            }

            /// Update entity
            pub fn update(conn: &rusqlite::Connection, entity: &$entity) -> anyhow::Result<()> {
                let mut field_assignments = Vec::new();
                $(field_assignments.push(format!("{} = ?", stringify!($field)));)*
                field_assignments.extend(vec!["metadata = ?".to_string(), "status = ?".to_string()]);

                let sql = format!(
                    "UPDATE {} SET {} WHERE id = ?",
                    $table,
                    field_assignments.join(", ")
                );

                let metadata_json = serde_json::to_string(&entity.metadata)?;
                let status_str = serde_json::to_string(&entity.status)?.trim_matches('"').to_string();

                conn.execute(&sql, rusqlite::params![
                    $(entity.$field,)*
                    metadata_json,
                    status_str,
                    entity.id
                ])?;

                Ok(())
            }

            /// Delete entity by ID
            pub fn delete(conn: &rusqlite::Connection, id: i32) -> anyhow::Result<()> {
                let sql = format!("DELETE FROM {} WHERE id = ?1", $table);
                conn.execute(&sql, rusqlite::params![id])?;
                Ok(())
            }

            /// Convert database row to entity
            fn row_to_entity(row: &rusqlite::Row) -> rusqlite::Result<$entity> {
                let mut col_idx = 0;
                
                let id = row.get(col_idx)?;
                col_idx += 1;

                $(
                    let $key_field = row.get(col_idx)?;
                    col_idx += 1;
                )+

                $(
                    let $field = row.get(col_idx)?;
                    col_idx += 1;
                )*

                let metadata_str: String = row.get(col_idx)?;
                let metadata: std::collections::HashMap<String, serde_json::Value> = 
                    serde_json::from_str(&metadata_str).map_err(|_| {
                        rusqlite::Error::InvalidColumnType(col_idx, "metadata".to_string(), rusqlite::types::Type::Text)
                    })?;
                col_idx += 1;

                let created_at_str: String = row.get(col_idx)?;
                let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(col_idx, "created_at".to_string(), rusqlite::types::Type::Text))?
                    .with_timezone(&chrono::Utc);
                col_idx += 1;

                let status_str: String = row.get(col_idx)?;
                let status_json = format!("\"{}\"", status_str);
                let status = serde_json::from_str(&status_json).map_err(|_| {
                    rusqlite::Error::InvalidColumnType(col_idx, "status".to_string(), rusqlite::types::Type::Text)
                })?;

                Ok($entity {
                    id,
                    $($key_field,)+
                    $($field,)*
                    metadata,
                    created_at,
                    status,
                })
            }
        }
    };
}

/// Macro 3: Generate business logic methods
#[macro_export]
macro_rules! define_entity_methods {
    (
        $entity:ident,
        $db_struct:ident,
        key_fields: { $($key_field:ident: $key_type:ty),+ },
        fields: { $($field:ident: $field_type:ty),* }
    ) => {
        impl $entity {
            /// Create new entity with set args
            pub fn create_new($($key_field: $key_type,)+ set_args: Vec<(String, String)>) -> anyhow::Result<Self> {
                let mut entity = Self {
                    id: 0,  // Will be set by database
                    $($key_field,)+
                    $($field: Default::default(),)*
                    metadata: std::collections::HashMap::new(),
                    created_at: chrono::Utc::now(),
                    status: Default::default(),
                };

                entity.process_set_args(set_args)?;
                Ok(entity)
            }

            /// Process --set arguments
            pub fn process_set_args(&mut self, set_args: Vec<(String, String)>) -> anyhow::Result<()> {
                for (key, value) in set_args {
                    match key.as_str() {
                        $(stringify!($field) => {
                            // Try to parse as number first, then as string
                            let json_value = if let Ok(num) = value.parse::<i32>() {
                                serde_json::Value::Number(serde_json::Number::from(num))
                            } else if let Ok(num) = value.parse::<f64>() {
                                serde_json::Value::Number(serde_json::Number::from_f64(num).unwrap())
                            } else {
                                serde_json::Value::String(value)
                            };
                            self.$field = serde_json::from_value(json_value)?;
                        })*
                        "status" => {
                            let json_str = format!("\"{}\"", value);
                            self.status = serde_json::from_str(&json_str)?;
                        }
                        // Relations handled in extension macro
                        _ => {
                            self.metadata.insert(key, serde_json::Value::String(value));
                        }
                    }
                }
                Ok(())
            }

            /// Save to database
            pub fn create(&mut self) -> anyhow::Result<()> {
                let _world_root = Self::ensure_world_context()?;
                let conn = Self::get_database_connection()?;
                $db_struct::init_table(&conn)?;
                
                self.id = $db_struct::insert(&conn, self)?;
                println!("✅ Created {} '{}'", stringify!($entity), self.display_key());
                Ok(())
            }

            /// Get entity by logical key
            pub fn get($($key_field: &$key_type),+) -> anyhow::Result<Option<Self>> {
                let _world_root = Self::ensure_world_context()?;
                let conn = Self::get_database_connection()?;
                $db_struct::get_by_key(&conn, $($key_field),+)
            }

            /// List all entities
            pub fn list() -> anyhow::Result<Vec<Self>> {
                let _world_root = Self::ensure_world_context()?;
                let conn = Self::get_database_connection()?;
                $db_struct::list(&conn)
            }

            /// Update entity with set args
            pub fn update(&mut self, set_args: Vec<(String, String)>) -> anyhow::Result<()> {
                self.process_set_args(set_args)?;
                let conn = Self::get_database_connection()?;
                $db_struct::update(&conn, self)?;
                println!("✅ Updated {} '{}'", stringify!($entity), self.display_key());
                Ok(())
            }

            /// Delete entity
            pub fn delete(&self, force: bool) -> anyhow::Result<()> {
                if !force {
                    anyhow::bail!("Use --force to confirm deletion");
                }
                let conn = Self::get_database_connection()?;
                $db_struct::delete(&conn, self.id)?;
                println!("✅ Deleted {} '{}'", stringify!($entity), self.display_key());
                Ok(())
            }

            /// Display key for user feedback  
            pub fn display_key(&self) -> String {
                // For single key, just return it. For composite keys, join with ':'
                let mut parts = Vec::new();
                $(parts.push(self.$key_field.to_string());)+
                parts.join(":")
            }

            // Utility methods
            fn get_database_connection() -> anyhow::Result<rusqlite::Connection> {
                let db_path = crate::world::WorldConfig::get_database_path()?;
                rusqlite::Connection::open(&db_path).map_err(|e| anyhow::anyhow!("Database connection failed: {}", e))
            }

            fn ensure_world_context() -> anyhow::Result<std::path::PathBuf> {
                crate::world::WorldConfig::get_world_root()
                    .map_err(|_| anyhow::anyhow!("Not in a multiverse project directory. Run 'multiverse world init <name>' to create one."))
            }
        }
    };
}

/// Macro 4: Complete entity (combines all above)
#[macro_export]
macro_rules! define_complete_entity {
    (
        $entity:ident,
        $status:ident,
        $db_struct:ident,
        table: $table:literal,
        key_fields: { $($key_field:ident: $key_type:ty),+ },
        fields: { $($field:ident: $field_type:ty),* },
        status_variants: [ $($variant:ident),+ ],
        create_sql: $sql:literal
    ) => {
        $crate::define_entity_database!(
            $db_struct,
            $entity,
            table: $table,
            key_fields: { $($key_field: $key_type),+ },
            fields: { $($field: $field_type),* },
            create_sql: $sql
        );

        $crate::define_entity_struct!(
            $entity,
            $status,
            key_fields: { $($key_field: $key_type),+ },
            fields: { $($field: $field_type),* },
            status_variants: [ $($variant),+ ]
        );


        $crate::define_entity_methods!(
            $entity,
            $db_struct,
            key_fields: { $($key_field: $key_type),+ },
            fields: { $($field: $field_type),* }
        );
    };
}
