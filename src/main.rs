mod core;
use crate::core::hasher::hash_u8_vec;
use crate::core::verifier::{verify_age, verify_country};
use crate::core::{Parameters, TrustedCenter, User};

mod circuits;
mod utils;

fn main() {
    let params = Parameters::new();

    let valid_age_range = 18..120;
    let valid_countries = vec!["America".to_string(), "Brazil".to_string()];

    let users = vec![
        User::new(21, "America"),
        User::new(14, "Brazil"),
        User::new(99, "Russia"),
    ];

    let mut tc = TrustedCenter::new();
    for user in users.iter() {
        tc.add_user(&user, params.age_params(), params.country_params());
    }

    for (idx, user) in users.iter().enumerate() {
        let user_id_hash = hash_u8_vec(user.id());

        match verify_age(&user_id_hash, &valid_age_range, &tc, params.age_params()) {
            true => println!("User {} has passed age check", idx + 1),
            false => println!("User {} has failed age check", idx + 1),
        }

        match verify_country(
            &user_id_hash,
            &valid_countries,
            &tc,
            params.country_params(),
        ) {
            true => println!("User {} has passed country check", idx + 1),
            false => println!("User {} has failed country check", idx + 1),
        }

        println!()
    }
}
