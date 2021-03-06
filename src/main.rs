/*
bin2dec: a small project to convert a binary string to a decimal number. easy enough :>
link: https://github.com/florinpop17/app-ideas/blob/master/Projects/1-Beginner/Bin2Dec-App.md

challenge constraints:
- arrays may not be used to contain the binary digits entered by the user
- determining the decimal equivalent of a particular binary digit in the sequence must be 
calculated using a single mathematical function, for example the natural logarithm. it's 
up to you to figure out which function to use.

user stories:
- user can enter up to 8 binary digits in one input field
- user must be notified if anything other than a 0 or 1 was entered
- user views the results in a single output field containing the decimal (base 10) equivalent 
binary number

bonus features:
- user can enter a variable number of digits
*/

fn bin_2_dec(binary_string: &str) -> i32 {
    let mut sum = 0;
    let mut characters = binary_string.trim_end().chars().rev();

    // find a way to strip escape chars so i dont have to reduce by 2
    for n in 0..binary_string.len() {
        if characters.next() == Some('1') {
            sum += num::pow(2, n as usize);
        }
    }

    println!("{}", sum);
    sum
}

fn main() {
    println!("Enter a binary number: (1's and 0's)");

    let mut binary = String::new();

    let _user_input = std::io::stdin().read_line(&mut binary);

    bin_2_dec(&binary);
}