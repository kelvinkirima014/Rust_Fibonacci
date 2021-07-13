use std:: io;
fn main() {
    println!(" To exit the program, type `exit` ");
    loop {
        // create a mutable variable and bound it to 
        //a new empty string
       let mut int = String::new();
       println!(" ENTER a POSITIVE INTEGER");
       //handle input in the terminal
       io::stdin()
           .read_line(&mut int)// gets input from user
           .expect("");//handle error with `Err`, or returns `ok`
        if int.trim()== "exit" {
            break; //  end loop
        }
        let int: u32 = match int.trim()
            .parse(){
                Ok (num) => num,
                Err(_) => continue,
            };
        
        println!("fibonacci ({}) => {}", int,fib(int));

    }
}
fn fib (n: u32) -> u32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n -2);
    }
}
