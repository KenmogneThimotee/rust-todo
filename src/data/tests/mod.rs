



#[cfg(test)]
mod status;

#[cfg(test)]
mod task{
    use clap::builder::Str;
    use dotenvy::dotenv;
    use std::env;
    use crate::data::DataBase;




    #[test]
    fn test_echo(){

        let mut database = DataBase::new(String::from(":memory:"));

        database.run_migrations();
        let status = database.create_status(String::from("Status"),String::from("Color")).unwrap();

        assert_eq!(status.name, String::from("Status"));
    }

}