use sqlx::Execute;

use crate::entity::{role::Role, user::User};

use super::JoinTable;

static DEFAULT_INSERT: &'static str = r#"INSERT INTO "SMS_GATEWAY_USER"."USER_ROLE" (user_id, role_id) VALUES ($1, $2)"#;

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

impl<'c> JoinTable<'c, User, Role> {

    pub async fn create_user_roles(&self, user_id: &i32, roles: &Vec<Role>) -> Result<u64, sqlx::Error> {

        if 0 == roles.len() {
            Ok(0)
        } else {
            let insert_statement = build_insert_statement(roles.len());
            dbg!("{:?}", &insert_statement);

            let mut query = sqlx::query(&insert_statement);

            for role in roles {
                query = query.bind(user_id).bind(role.role_id);
            }

            dbg!("{:?}", &query.sql());

            query.execute(&*self.pool).await
                .map(|x| x.rows_affected())
        }
    }

    pub async fn find_user_roles(&self, user_id: &i32) -> Result<Vec<Role>, sqlx::Error> {
        let user = sqlx::query_as!(User, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."USER" WHERE user_id = $1 "#, user_id)
            .fetch_one(&*self.pool)
            .await?;

        sqlx::query_as!(Role, 
            r#"SELECT * FROM "SMS_GATEWAY_USER"."ROLE" p WHERE p.role_id IN (SELECT r.role_id FROM "SMS_GATEWAY_USER"."USER_ROLE" r WHERE r.user_id = $1)"#, 
            user.user_id)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn update_user_roles(&self, role_id: &i32, roles: &Vec<Role>) -> Result<u64, sqlx::Error> {
        if roles.len() == 0 {
            self.delete_user_roles(role_id).await
        } else {
            let deleted = self.delete_user_roles(role_id).await?;
            let added = self.create_user_roles(role_id, roles).await?;
            Ok(added + deleted)
        }
    }

    pub async fn delete_user_roles(&self, user_id: &i32) -> Result<u64, sqlx::Error> {
        sqlx::query_as!(PgQueryResult, 
            r#"DELETE FROM "SMS_GATEWAY_USER"."USER_ROLE" WHERE user_id = $1 "#, user_id)
            .execute(&*self.pool)
            .await.map(|x|x.rows_affected())
    }
}
