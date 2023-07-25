use std::num::ParseFloatError;



fn main() {
    println!("Hello, world!");
    
    let mut user_input : String = String::new();

    loop{
        println!("Please enter arithmetic operation formatted as number1 operation number 2. To quit please enter x.");
        std::io::stdin().read_line(&mut user_input).unwrap();
        if user_input.trim() == "x" {
            println!("Closing the calculator.");
            break;
        }
        //print for debugging purposes.
        //println!("User input is {:?}",user_input.trim());
        let user_operation:Result<Operation,&str> = parse_string_into_operation(&user_input);
        let operation_result : Result<f64,String> = match user_operation {
            Ok(user_requested_operation) => calculate(user_requested_operation),
            Err(msg) =>{Err(msg.to_string())},
        };
    
        match operation_result{
            Ok(val) => {println!("Value is {}",val)},
            Err(error_message) => {println!("Error message : {}",error_message)},
        }; 
        user_input.clear();
    }
    
}

fn parse_string_into_operation(user_input : &str) -> Result<Operation, &str>{

    let operators: [char;4] = ['+','/','-','*'];

    let  first_number_as_string :String;
    let  second_number_as_string :String;
    let mut operation :char = ' ';

    for character in user_input.chars() {
        if operators.contains(&character){
            operation = character;
            break;
        }
    }
    if operation == ' '
    {
        return Err("Could not find a suitable operand, please use + for add, - for substract, / for divide and * for multiplying.");
    }
    let split_input:Vec<&str> = user_input.split(operation).collect();

    first_number_as_string = split_input[0].trim().to_string();
    second_number_as_string = split_input[1].trim().to_string();

    let built_operation: Result<Operation,&str>;
    //debug prints
    //println!("first number as string :{:?}",first_number_as_string);
    //println!("first number as string :{:?}",second_number_as_string);

    let  first_number_as_float :Result<f64,ParseFloatError> = first_number_as_string.parse();
    let  second_number_as_float :Result<f64,ParseFloatError> = second_number_as_string.parse();


    if first_number_as_float.is_err()
    {
        return Err("Could not convert first number to float, please check your input and try again.");
    }
    if second_number_as_float.is_err()
    {
        return Err("Could not convert second number to float, please check your input and try again.");
    }

    built_operation = match operation{
        '+' => Ok(Operation::Add { first_number: first_number_as_float.unwrap(), second_number: second_number_as_string.parse().unwrap() }),
        '-' => Ok(Operation::Substract{ first_number: first_number_as_float.unwrap(), second_number: second_number_as_string.parse().unwrap() }),
        '/' => Ok(Operation::Divide  { first_number: first_number_as_float.unwrap(), second_number: second_number_as_string.parse().unwrap() }),
        '*' => Ok(Operation::Multiply { first_number: first_number_as_float.unwrap(), second_number: second_number_as_string.parse().unwrap() }),
        _ => Err("Operand is not correct, please enter operand as one of + - / * ."),
    };
    //debug print
    //println!("Built operation is : {:?}",built_operation);

    built_operation
}
#[derive(Debug)]
enum Operation{
    Add{first_number:f64,second_number:f64},
    Substract{first_number:f64,second_number:f64},
    Divide{first_number:f64,second_number:f64},
    Multiply{first_number:f64,second_number:f64},
}

fn calculate(operation_to_be_performed:Operation) -> Result<f64,String>{
    match operation_to_be_performed {
        Operation::Add { first_number, second_number } => {
            Ok(first_number + second_number)
        }
        Operation::Divide { first_number, second_number } =>{
            if second_number == 0.0{
                Err(String::from("Divide by zero error"))
            }
            else{
                Ok(first_number / second_number)
            }
        }
        Operation::Multiply { first_number, second_number } =>{
            Ok(first_number * second_number)
        }
        Operation::Substract { first_number, second_number } =>{
            Ok(first_number - second_number)
        }
    
    }
}
