use regex::Regex;


fn _is_valid_email(email: &String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}

// fn vaildate_user(new_user: CreateUser) -> Result<CreateUser,String>{
//     if new_user.name.trim().is_empty()  {
//         return Err("User name is empty".to_string());
//     } else if is_valid_email(&new_user.email)  {
//         return Err("Invalid email".to_string());
//     } else if new_user.password.trim().is_empty()  {
//         return Err("password is empty".to_string());
//     }

//     Ok(new_user)
// }