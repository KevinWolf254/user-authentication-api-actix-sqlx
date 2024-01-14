use sqlx::Execute;

use crate::entity::{role::Role, permission::Permission};

use super::JoinTable;

static DEFAULT_INSERT: &'static str = r#"INSERT INTO "SMS_GATEWAY_USER"."ROLE_PERMISSION" (role_id, permission_id) VALUES ($1, $2)"#;

fn build_insert_statement(rows: usize) -> String {
    let mut insert = String::from(DEFAULT_INSERT);

    match rows {
        1 | 0 => insert,
        _ => {
            let mut i = 1;
            let mut params = 3;
            while i < rows {
                insert.push_str(", ($");
                insert.push_str(&params.to_string());
                insert.push_str(", $");
                insert.push_str(&(params + 1).to_string());
                insert.push_str(")");
                i += 1;
                params += 2;
            }
            insert
        }
    }
}

impl<'c> JoinTable<'c, Role, Permission> {

    pub async fn create_role_permissions(&self, role_id: &i16, permissions: &Vec<Permission>) -> Result<u64, sqlx::Error> {

        if 0 == permissions.len() {
            Ok(0)
        } else {
            let insert_statement = build_insert_statement(permissions.len());
            dbg!("{:?}", &insert_statement);

            let mut query = sqlx::query(&insert_statement);

            for permission in permissions {
                query = query.bind(role_id).bind(permission.permission_id);
            }

            dbg!("{:?}", &query.sql());

            query.execute(&*self.pool).await
                .map(|x| x.rows_affected())
        }
    }

    pub async fn find_role_permissions(&self, role_id: &i16) -> Result<Vec<Permission>, sqlx::Error> {
        let role = sqlx::query_as!(Role, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."ROLE" WHERE role_id = $1 "#, role_id)
            .fetch_one(&*self.pool)
            .await?;

        sqlx::query_as!(Permission, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."PERMISSION" p WHERE p.permission_id IN (SELECT r.permission_id FROM "SMS_GATEWAY_USER"."ROLE_PERMISSION" r WHERE r.role_id = $1)"#, 
            role.role_id)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn update_role_permissions(&self, role_id: &i16, permissions: &Vec<Permission>) -> Result<u64, sqlx::Error> {
        if permissions.len() == 0 {
            self.delete_role_permissions(role_id).await
        } else {
            let deleted = self.delete_role_permissions(role_id).await?;
            let added = self.create_role_permissions(role_id, permissions).await?;
            Ok(added + deleted)
        }
    }

    pub async fn delete_role_permissions(&self, role_id: &i16) -> Result<u64, sqlx::Error> {
        sqlx::query_as!(PgQueryResult, 
            r#"DELETE FROM "SMS_GATEWAY_USER"."ROLE_PERMISSION" WHERE role_id = $1 "#, role_id)
            .execute(&*self.pool)
            .await.map(|x|x.rows_affected())
    }
}
