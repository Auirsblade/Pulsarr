use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use sqlx::types::Decimal;
use sqlx::{FromRow, Postgres, query_as};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::QueryAs;
use crate::data::models::Model;

#[derive(Serialize, Deserialize, FromRow, JsonSchema)]
pub(crate) struct RatingSystemTemplateParameter {
    pub template_parameter_id: i32,
    pub template_id: i32,
    pub name: String,
    pub parameter_rating_max: Decimal,
    pub weight: Decimal,
}

impl Model for RatingSystemTemplateParameter {
    fn add<RatingSystemTemplateParameter: for<'r> sqlx::FromRow<'r, PgRow>>(self) -> QueryAs<'static, Postgres, RatingSystemTemplateParameter, PgArguments> {
        query_as(
            "INSERT INTO rating_system_template_parameter (template_id, name, parameter_rating_max, weight)\
            VALUES ($1, $2, $3, $4)\
            RETURNING *",
        )
            .bind(self.template_id)
            .bind(self.name)
            .bind(self.parameter_rating_max)
            .bind(self.weight)
    }

    fn update<T: Model>(self) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as(
            "UPDATE rating_system_template_parameter \
            SET template_id = $2, name = $3, parameter_rating_max = $4, weight = $5 \
            WHERE template_parameter_id = $1 \
            RETURNING *"
        )
            .bind(self.template_parameter_id)
            .bind(self.template_id)
            .bind(self.name)
            .bind(self.parameter_rating_max)
            .bind(self.weight)
    }

    fn delete<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("DELETE FROM rating_system_template_parameter WHERE template_parameter_id = $1").bind(id)
    }

    fn get_by_id<T: Model>(id: i32) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_system_template_parameter WHERE template_parameter_id = $1").bind(id)
    }

    fn get_all<T: Model>(_take_size: Option<i32>, _offset: Option<i32>) -> QueryAs<'static, Postgres, T, PgArguments> {
        query_as("SELECT * FROM rating_system_template_parameter")
    }
}

pub fn get_by_template_id(id: i32) -> QueryAs<'static, Postgres, RatingSystemTemplateParameter, PgArguments> {
    query_as("SELECT * FROM rating_system_template_parameter WHERE template_id = $1").bind(id)
}
