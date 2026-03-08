fn main() {

    let d1 = get_positive_division(5, 2); 
    
    println!("{:?}", d1); 
    
    println!("{:?}", get_positive_division(5, 0)); 
    
    println!("{:?}", get_positive_division(-5, -3)); 
    
    println!("{:?}", get_positive_division(-5, 3)); 

}

#[derive(Debug)]
enum Errors {
    
    ZeroDivisionError(&'static str),
    TypeSelectionError(&'static str),
   
}

fn get_positive_division(x: i64, y: i64) -> Result<f64, Errors> {

    if y == 0 {

        Err(Errors::ZeroDivisionError("Divisor value can't be zero!"))

    } 
    
    else if (x < 0 && y > 0) || (y < 0 && x > 0) { // else if (x * y < 0)
        
        Err(Errors::TypeSelectionError("The numbers must be both positive or must be both negative!"))
    }
    
    else {
        
        Ok(x as f64 / y as f64)
    }
    
    /*
    
Ok(2.5)
Err(ZeroDivisionError("Divisor value can't be zero!"))
Ok(1.6666666666666667)
Err(TypeSelectionError("The numbers must be both positive or must be both negative!"))

    */
}
