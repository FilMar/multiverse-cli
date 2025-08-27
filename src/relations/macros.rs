//! Composable macro system for generating relation types, parsers, and processors
//! This eliminates boilerplate code for defining new relation types
/// Macro to define just the struct and basic implementation
#[macro_export]
macro_rules! define_relation_struct {
    (
        $relation_name:ident,
        {
            $($field:ident: $field_type:ty),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $relation_name {
            $(pub $field: $field_type),+,
            pub created_at: String,
        }

        impl $relation_name {
            pub fn new($($field: $field_type),+) -> Self {
                Self {
                    $($field),+,
                    created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                }
            }
        }
    };
}

/// Macro to implement the Relation trait
#[macro_export]
macro_rules! impl_relation_trait {
    (
        $relation_name:ident,
        table: $table_name:literal,
        create: $create_sql:literal,
        key_fields: {
            $key1:ident: $key_type1:ty,
            $key2:ident: $key_type2:ty
        },
        update_fields: [$($update_field:ident),+],
        db_struct: $db_struct:ident
    ) => {
        impl crate::relations::models::Relation for $relation_name {
            fn create(&self, conn: &rusqlite::Connection) -> anyhow::Result<()> {
                $db_struct::insert(conn, 
                    self.$key1.clone(), 
                    &self.$key2, 
                    $(&self.$update_field),+
                )
            }

            fn update(&self, conn: &rusqlite::Connection) -> anyhow::Result<()> {
                $db_struct::update(conn, 
                    self.$key1.clone(), 
                    &self.$key2, 
                    $(&self.$update_field),+
                )
            }

            fn delete(&self, conn: &rusqlite::Connection) -> anyhow::Result<()> {
                $db_struct::delete(conn, self.$key1.clone(), &self.$key2)
            }
        }
    };
}

/// Macro to generate parser functions - simplified for generic relations
#[macro_export]
macro_rules! define_relation_parser {
    (
        $parser_name:ident,
        $relation_struct:ident,
        format: $format:literal
    ) => {
        #[derive(Debug)]
        pub struct $relation_struct {
            pub first: String,
            pub second: String,
            pub third: String,
        }

        pub fn $parser_name(value: &str) -> anyhow::Result<Vec<$relation_struct>> {
            let mut relations = Vec::new();
            
            for part in value.split(',') {
                let components: Vec<&str> = part.trim().split(':').collect();
                
                if components.is_empty() {
                    return Err(anyhow::anyhow!("Invalid format: '{}'. Expected {}", part, $format));
                }
                
                let first = components[0].to_string();
                let second = components.get(1).unwrap_or(&"").to_string();
                let third = components.get(2).unwrap_or(&"").to_string();
                
                relations.push($relation_struct {
                    first,
                    second,
                    third,
                });
            }
            
            Ok(relations)
        }
    };
}

/// Macro to generate processor function - uses the same parameters from complete_relation
#[macro_export]
macro_rules! define_relation_processor {
    (
        $processor_name:ident,
        $parser_name:ident,
        $relation_name:ident,
        init_fn: $init_fn:path,
        key_fields: {
            $($key_name:ident: $key_type:ty),+
        },
        fields: {
            $($field_name:ident: $field_type:ty),+
        }
    ) => {
        pub fn $processor_name(entity_id: &str, value: &str) -> anyhow::Result<()> {
            let db_path = crate::world::WorldConfig::get_database_path()?;
            let conn = crate::database::get_connection(&db_path)?;
            
            $init_fn(&conn)?;
            
            let relations = $parser_name(value)?;
            
            for rel in relations {
                // Create relation with proper parameter mapping
                // This assumes: first key = entity_id, second key = rel.first, then additional fields
                let relation_obj = $relation_name::new(
                    entity_id.parse().unwrap_or_else(|_| entity_id.to_string() as _),  // Try parse or use as string
                    rel.first.clone(),      
                    rel.second.clone(),     
                    rel.third.clone()       
                );
                
                match relation_obj.create(&conn) {
                    Ok(_) => {
                        println!("✅ Added relation: {} ↔ {}", entity_id, rel.first);
                    }
                    Err(_) => {
                        relation_obj.update(&conn)?;
                        println!("✅ Updated relation: {} ↔ {}", entity_id, rel.first);
                    }
                }
            }
            
            Ok(())
        }
    };
}

/// Complete macro that uses all the composable parts
#[macro_export]
macro_rules! define_complete_relation {
    (
        $relation_name:ident,
        table: $table_name:literal,
        key_fields: {
            $key1:ident: $key_type1:ty,
            $key2:ident: $key_type2:ty
        },
        fields: {
            $($field:ident: $field_type:ty),+ $(,)?
        },
        sql: $create_sql:literal,
        update_fields: [$($update_field:ident),+],
        parser: {
            name: $parser_name:ident,
            struct: $relation_struct:ident,
            format: $format:literal
        },
        processor: {
            name: $processor_name:ident,
            init_fn: $init_fn:path
        },
        db_struct: $db_struct:ident
    ) => {
        $crate::define_relation_struct!($relation_name, { 
            $key1: $key_type1,
            $key2: $key_type2,
            $($field: $field_type),+ 
        });
        
        $crate::impl_relation_trait!($relation_name, 
            table: $table_name, 
            create: $create_sql,
            key_fields: {
                $key1: $key_type1,
                $key2: $key_type2
            },
            update_fields: [$($update_field),+],
            db_struct: $db_struct
        );
        
        $crate::define_relation_parser!($parser_name, $relation_struct, format: $format);
        
        $crate::define_relation_processor!($processor_name, $parser_name, $relation_name, 
            init_fn: $init_fn,
            key_fields: {
                $key1: $key_type1,
                $key2: $key_type2
            },
            fields: {
                $($field: $field_type),+
            }
        );
    };
}

// Macro exports are handled by #[macro_export] - no manual re-exports needed
