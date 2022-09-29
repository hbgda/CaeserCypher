const CHAR_COUNT: u32 = 26;
const CHAR_START: u32 = 'a' as u32;
const CHAR_END: u32 = CHAR_START + CHAR_COUNT;


fn main() {

    println!("Input: Format <method[decrypt|encrypt]>;<offset>;<source> e.g. encrypt;12;this is cool");

    let mut input_string: String = String::new();
    // Read user input, reads into the mutable `input_string` variable
    std::io::stdin().read_line(&mut input_string)
        .expect("Error.");
    
    // Parse input
    let (method, offset, source) = parse_input(input_string);

    println!("\nMethod: {method}\nOffset: {offset}\nSource: {source}\n");

    if method == "encrypt" {
        // Encrypt string
        let encrypted_string: String = encrypt_string(&source, offset);
        println!("Encrypted: {encrypted_string}")
    }
    else {
        // Decrypt string
        let decrypted_string: String = decrypt_string(&source, offset);
        println!("Decrypted: {decrypted_string}")
    }
}

// Parse user input into a (String, u32, String) tuple containing the method, offset and source string
// This is done by splitting the provided input by a delimiter, in this case the ;
fn parse_input(input: String) -> (String, u32, String) {
    let input_split: Vec<&str> = input.split(";").collect();

    if input_split[0] != "encrypt" && input_split[0] != "decrypt" {
        eprintln!("Invalid method, does not match encrypt|decrypt.");
        panic!();
    }
    
    let offset_str: &str = input_split[1];

    // Attempt to parse `offset_str` to a u32
    let offset: u32 = {
        let mut _offset: u32 = offset_str.trim_start_matches("-").parse::<u32>()
            .expect(format!("Invalid value passed for `offset`: {offset_str}").as_str());
        
        if offset_str.starts_with("-") {
            _offset = CHAR_COUNT - bound_offset(_offset);
        }

        bound_offset(_offset)
    };
      
                        // Convert to `String` due to restrictions of `&str` in return types.
    return (input_split[0].to_string(), offset, input_split[2..].join(";").to_string())
}

fn encrypt_string(source: &str, offset: u32) -> String {
    return get_offset_string(source, offset);
}

fn decrypt_string(source: &str, offset: u32) -> String {
                                    // Use `CHAR_COUNT` - offset as a way to subtract through overflow
                                    // since u32 (unsigned) cannot be negative
    return get_offset_string(source, CHAR_COUNT - offset);
}

// Offset the given `source` &str by iterating over its individual characters
// and appending the correspending offset character to a `Vec<char>` that can then be collected into a String.
fn get_offset_string(source: &str, offset: u32) -> String {

    // Vector to contain the offset characters
    let mut offset_char_vec: Vec<char> = Vec::new();

    for _char in source.chars() {

        // Check if `_char` is within the bounds of the encoder, if not just push it to the vector as is.
        if (_char as u32) > CHAR_END || (_char as u32) < CHAR_START {
            offset_char_vec.push(_char);
            continue;
        }

        // Get offset char
        let offset_char: char = get_offset_char(_char, offset);
        
        println!("Source Char: {_char} | Offset Char: {offset_char}");

        // Push offset char to the vector.
        offset_char_vec.push(offset_char);
    }

    // Consume the vector through `.into_iter()` then `.collect()` the result into a String
    return offset_char_vec.into_iter().collect();
}

// Get char at the given `offset` from `source`
fn get_offset_char(source: char, offset: u32) -> char {
    // println!("\nget_offset_char('{c}', {offset})\n");

    return std::char::from_u32(CHAR_START + (
        ((source as u32 - CHAR_START) + offset) % 26
    )).unwrap();

    // -------------
    //   OLD | BAD
    // -------------
    // // Convert `source` to u32 for calculations.
    // let char_as_u32: u32 = source as u32;

    // // Calculate characters betwwen the `source` and `CHAR_END`.
    // let remaining_chars: u32 = CHAR_END - char_as_u32 - 1;

    // // If `offset` is within the remaining characters it can be safely added to `char_as_u32`
    // // and converted back to the char type.
    // if offset <= remaining_chars {
    //     return std::char::from_u32(char_as_u32 + offset).unwrap();
    // }
    // else {
    //     return std::char::from_u32(CHAR_START + (offset - remaining_chars) - 1).unwrap();
    // }


}

// Ensure `offset` is within the bounds of `CHAR_COUNT`
fn bound_offset(offset: u32) -> u32 {
    return offset % CHAR_COUNT;
}