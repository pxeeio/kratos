pub(self) mod base;
pub mod exercise;
pub mod exercise_equipment;
pub mod exercise_instruction;
pub mod exercise_muscle_map;
mod errors;
pub mod link;
pub mod muscle;
pub mod muscle_group;
pub mod user;
pub mod profile;

pub use exercise::{CreateExerciseData, Exercise};
pub use exercise_equipment::ExerciseEquipment;
pub use exercise_instruction::ExerciseInstruction;
pub use exercise_muscle_map::{CreateExerciseMuscleMapData, ExerciseMuscleMap};
pub use errors::Error;
pub use link::{CreateLinkData, Link};
pub use muscle::{CreateMuscleData, Muscle, MuscleRecord};
pub use muscle_group::MuscleGroup;
pub use profile::{CreateUserProfileData, Profile};
pub use user::{NewUser, User};
pub(self) type Result<TValue> = ::core::result::Result<TValue, Error>;

use crate::sys::DatabaseManager;
use async_trait::async_trait;
use sqlx::{
    postgres::PgRow,
    Encode, FromRow, PgPool, Postgres, Type,
};

#[async_trait]
pub trait Model
where
    Self: Send + Sized + Unpin,
{
    const ROUTE_KEY: &'static str = "id";
    const MODEL_NAME: &'static str;
    const TABLE_NAME: &'static str;
    type Attributes: for<'r> FromRow<'r, PgRow> + Unpin + Send;

    fn connection(&self) -> &PgPool;
    fn from_database(attributes: Self::Attributes, database: &DatabaseManager) -> Self;

    async fn find<TKey>(key: &'static str, value: TKey, database: &DatabaseManager) -> Result<Self>
    where
        TKey: Type<Postgres> + for<'q> Encode<'q, Postgres> + Send + std::fmt::Display + Clone,
    {
        base::find(key, value, database).await
    }

    async fn find_by_id(id: i64, database: &DatabaseManager) -> Result<Self> {
        base::find_by_id(id, database).await
    }

    async fn find_by_key<TKey>(value: TKey, database: &DatabaseManager) -> Result<Self>
    where
        TKey: Type<Postgres> + for<'q> Encode<'q, Postgres> + Send + std::fmt::Display + Clone,
    {
        base::find(Self::ROUTE_KEY, value, database).await
    }

    async fn all(database: &DatabaseManager) -> Result<Vec<Self>> {
        base::all(database).await
    }

    async fn count(database: &DatabaseManager) -> Result<i64> {
        let row_count = sqlx::query_as::<_, (i64,)>(format!("SELECT count(*) FROM {}", Self::TABLE_NAME).as_str())
            .fetch_one(database.connection())
            .await?;

        Ok(row_count.0)
    }
}
