fn main() {
    let my_first_string = String::from("Hello, ");
    let my_second_string = String::from("Patika.dev!");

    let my_first_ref_to_string: &str = &my_first_string;
    let my_second_ref_to_string: &str = &my_second_string;

    let my_final_string: String =
        concatenate_strings(my_first_ref_to_string, my_second_ref_to_string);

    println!("final string is : {}", my_final_string);
}

fn concatenate_strings(first_string_ref: &str, my_second_string_ref: &str) -> String {
    let mut my_final_string = String::from("");
    my_final_string.push_str(first_string_ref);
    my_final_string.push_str(my_second_string_ref);

    my_final_string
}
