pub use sea_orm_migration::prelude::*;

mod m20230404_143730_user;
mod m20230404_143746_permision;
mod m20230404_143819_class;
mod m20230404_143932_note;
mod m20230404_153204_mark;
mod m20230404_153304_attendance;
mod m20230404_172304_lesson;
mod m20230405_163933_user_permision;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230404_143746_permision::Migration),
            Box::new(m20230405_163933_user_permision::Migration),
            Box::new(m20230404_143730_user::Migration),
            Box::new(m20230404_143819_class::Migration),
            // Box::new(m20230404_143932_note::Migration),
            // Box::new(m20230404_153204_mark::Migration),
            // Box::new(m20230404_153304_attendance::Migration),
            // Box::new(m20230404_172304_lesson::Migration),
        ]
    }
}
