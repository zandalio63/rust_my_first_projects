enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division
}

impl Operation {
   fn operation(&self, num1 : f64, num2 : f64) -> Result<f64, String> {
        match self  {
            Self::Addition => Ok(num1 + num2),
            Self::Subtraction =>Ok(num1 - num2),
            Self::Multiplication => Ok(num1 * num2),
            Self::Division => {
                if num2 == 0.0 {
                    return Err("No se puede dividir entre 0".to_string());
                }

                Ok(num1 / num2)
            },
        }
   }

   fn print_result(result : Result<f64, String>){
        match result {
            Ok(value) => println!("Resultado: {}", value),
            Err(err) => println!("Error {}", err),
        }
   }
}

fn main() {
    let addition = Operation::Addition.operation(4.0, 4.0) ;
    let subtraction = Operation::Subtraction.operation(6.0, 8.0); 
    let multiplication = Operation::Multiplication.operation(5.0, 3.0);
    let division = Operation::Division.operation(7.0, 0.0);

    Operation::print_result(addition);
    Operation::print_result(subtraction);
    Operation::print_result(multiplication);
    Operation::print_result(division);
}
