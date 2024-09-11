use rand::Rng;

    // generate a password: rp -p [-an] [-s] [-<length>] [-c] 
       // -[an] -> alpha or numeric or both 
       // -[s] -> special characters  
       // -<length> -> length of the password (default is 12 characters) 
       // -c -> copy to clipboard  
    // Find  
    // Add  
    // Remove 

// .password_store 
    // sites 
    // website
    //      

// Constants 
const LOWER_ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER_ALPHA: &[u8] = b"ABCDEGHIJKLMNOPQRTSTUVWXYZ";    
const NUMERALS: &[u8] = b"1234567890";
const SPECIAL: &[u8] = b"!@#$%^&*()[]{}:;"; 
const CHARSET: &[&[u8]] = &[LOWER_ALPHA, UPPER_ALPHA, NUMERALS, SPECIAL]; 

fn generate_password (alpha: bool, capital: bool, numeric: bool, special: bool, length: usize) -> String {
    // generate a password from a set of options 
    //      alpha, numeric, alpha-numeric 
    //      special characters (y / n) 
    //      Capitalization  if (alpha / alphanumeric)  
    //      Length of password     
    
    
    let mut choices: Vec<usize> = Vec::new(); 

    if alpha == true {
        choices.push(0);   
    }   
    
    if capital == true {
        choices.push(1); 
    }
    if numeric == true {
        choices.push(2);
    } 

    if special == true {
        choices.push(3);
    } 
      
    let mut rng = rand::thread_rng();  
    let password = (0..length)
        .map(|_| {
            let i = rng.gen_range(0..choices.len()); 
            let j = rng.gen_range(0..CHARSET[i].len());  
            CHARSET[choices[i]][j] as char
        })
        .collect(); 
    return password;

}
fn main() {
    let res: String = generate_password(true, true, true, true, 24);  
    println!("{}", res);
}
