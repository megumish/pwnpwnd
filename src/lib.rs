#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_debug_implementations)]

//! # pwnpwnd
//! 
//! pwnpwnd is a fast ctf competition envrionment library.
//!

extern crate dotenv;
extern crate envy;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct EnvOK {
    ok: String,
}

#[cfg(test)]
mod tests {
    use ::*;
    use dotenv;
    use envy;

    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn env_test() {
        dotenv::dotenv().expect("Failed to read .env file");
        match envy::from_env::<EnvOK>() {
            Ok(env_ok) => assert_eq!(env_ok.ok, "OK"),
            Err(err) => {
                println!("Read Env Error: {}", err);
                assert!(false);
            }
        }
    }
}
