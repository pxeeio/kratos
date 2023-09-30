use super::{Error, User};
use crate::enums::Gender;
use crate::sys::DatabaseManager;
use crate::types::ISO8601DateTimeUTC;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use ulid::Ulid;

#[derive(Clone, Debug, FromRow)]
pub struct UserProfileRecord {
    id: i64,
    ulid: String,
    user_id: i64,
    birthday: chrono::NaiveDate,
    gender: Gender,
    created_at: ISO8601DateTimeUTC,
    updated_at: ISO8601DateTimeUTC,
}

pub struct CreateUserProfileData {
    pub user_id: i64,
    pub birthday: chrono::NaiveDate,
    pub gender: Gender,
}

pub struct Profile {
    database: DatabaseManager,
    data: UserProfileRecord,
}

impl super::Model for Profile {
    const TABLE_NAME: &'static str = "user_profiles";
    type Attributes = UserProfileRecord;

    fn connection(&self) -> &PgPool {
        self.database.connection()
    }

    fn from_database(attributes: Self::Attributes, database: &DatabaseManager) -> Self {
        Self {
            database: database.clone(),
            data: attributes,
        }
    }
}

impl Profile {
    pub fn route_key(&self) -> String {
        self.data.ulid.clone()
    }

    pub fn birthday(&self) -> chrono::NaiveDate {
        self.data.birthday
    }

    pub fn gender(&self) -> Gender {
        self.data.gender.clone()
    }

    pub async fn create(
        attributes: CreateUserProfileData,
        database: &DatabaseManager,
    ) -> Result<Profile, sqlx::error::Error> {
        let mut transaction = database.connection().begin().await?;

        let ulid = Ulid::new();

        let profile = sqlx::query_as::<_, UserProfileRecord>("INSERT INTO user_profiles (ulid, user_id, birthday, gender) VALUES ($1, $2, $3, $4) RETURNING *")
            .bind(ulid.to_string().to_lowercase())
            .bind(attributes.user_id)
            .bind(attributes.birthday)
            .bind(attributes.gender)
            .fetch_one(&mut *transaction)
            .await;

        match profile {
            Ok(profile) => {
                transaction.commit().await?;
                Ok(Profile {
                    database: database.clone(),
                    data: profile,
                })
            }
            Err(err) => {
                transaction.rollback().await?;
                Err(err)
            }
        }
    }

    pub async fn find_by_user(
        user_id: i64,
        database: &DatabaseManager,
    ) -> Result<Profile, Error> {
        super::base::find::<Self, i64>("user_id", user_id, database).await
    }

    pub async fn find_by_ulid(
        ulid: String,
        database: &DatabaseManager,
    ) -> Result<Profile, Error> {
        super::base::find::<Self, String>("ulid", ulid, database).await
    }
}
