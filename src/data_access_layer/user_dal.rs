use actix_web::web;
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::my_errors::sqlite_errors::map_sqlite_error;
use crate::my_errors::sqlite_errors::SqliteError;
use crate::service_layer::user_service::{CreateUserRequest, UpdateUserInfosReq};
use crate::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub pseudo: String,
    pub password: String,
    pub email: String,
    pub age: u8,
    pub latitude: f32,
    pub longitude: f32,
    pub gender: String,
    pub looking_for: String,
    pub search_radius: u16
}

impl User {
    pub fn create_user(
        db: &web::Data<AppState>,
        user: CreateUserRequest,
    ) -> Result<(), SqliteError> {
        let mut statement = db
            .connection
            .prepare("INSERT INTO users (pseudo, password, email, age, latitude, longitude, gender, looking_for) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .map_err(map_sqlite_error)?;
        statement
            .execute(params![user.pseudo, user.password, user.email, user.age, user.latitude, user.longitude, user.gender, user.looking_for])
            .map_err(map_sqlite_error)?;

        Ok(())
    }

    pub fn get_user_by_email(
        db: &web::Data<AppState>,
        email: String,
    ) -> Result<User, SqliteError> {
        let mut statement = db
            .connection
            .prepare_cached("SELECT * FROM users WHERE email = ?")
            .map_err(map_sqlite_error)?;

        let user_found = statement
            .query_row(params![email], |row| {
                Ok(User {
                    id: row.get("person_id")?,
                    pseudo: row.get("pseudo")?,
                    email: row.get("email")?,
                    password: row.get("password")?, // TODO : DO NOT SEND BACK THE PASSWORD
                    age: row.get("age")?,
                    latitude: row.get("latitude")?,
                    longitude: row.get("longitude")?,
                    gender: row.get("gender")?,
                    looking_for: row.get("looking_for")?,
                    search_radius: row.get("search_radius")?,
                })
            })
            .map_err(map_sqlite_error)?;

        Ok(user_found)
    }

    pub fn get_user_by_id(db: &web::Data<AppState>, userId: u32) -> Result<User, SqliteError> {
        let mut statement = db
            .connection
            .prepare_cached("SELECT * FROM users WHERE person_id = ?")
            .map_err(map_sqlite_error)?;

        let user_found = statement
            .query_row(params![userId], |row| {
                Ok(User {
                    id: row.get("person_id")?,
                    pseudo: row.get("pseudo")?,
                    password: row.get("password")?,
                    email: row.get("email")?,
                    age: row.get("age")?,
                    latitude: row.get("latitude")?,
                    longitude: row.get("longitude")?,
                    gender: row.get("gender")?,
                    looking_for: row.get("looking_for")?,
                    search_radius: row.get("search_radius")?,
                })
            })
            .map_err(map_sqlite_error)?;

        Ok(user_found)
    }

    pub fn update_user_infos(
        db: &web::Data<AppState>,
        user: UpdateUserInfosReq,
    ) -> Result<(), SqliteError> {
        println!("{:?}", user);
        let mut statement = db
            .connection
            .prepare_cached(
                "UPDATE users
                SET pseudo = ?,
                email = ?,
                age = ?,
                latitude = ?,
                longitude = ?,
                gender = ?,
                looking_for = ?,
                search_radius = ?
                WHERE person_id = ?",
            )
            .map_err(map_sqlite_error)?;

        statement
            .execute(params![user.pseudo, user.email, user.age, user.latitude, user.longitude, user.gender, user.looking_for, user.search_radius, user.id])
            .map_err(map_sqlite_error)?;

        Ok(())
    }

    pub fn get_users(db: &web::Data<AppState>) -> Result<Vec<User>, SqliteError> {
        let mut statement = db
            .connection
            .prepare("SELECT * FROM users")
            .map_err(map_sqlite_error)?;
        let result_rows = statement
            .query_map([], |row| {
                Ok(User {
                    id: row.get("person_id")?,
                    pseudo: row.get("pseudo")?,
                    password: row.get("password")?,
                    email: row.get("email")?,
                    age: row.get("age")?,
                    latitude: row.get("latitude")?,
                    longitude: row.get("longitude")?,
                    gender: row.get("gender")?,
                    looking_for: row.get("looking_for")?,
                    search_radius: row.get("search_radius")?,
                })
            })
            .map_err(map_sqlite_error)?;

        let mut persons = Vec::new();
        for person in result_rows {
            persons.push(person.map_err(map_sqlite_error)?);
        }

        Ok(persons)
    }
}
