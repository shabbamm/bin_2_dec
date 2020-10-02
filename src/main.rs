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

fn bin_2_dec(range: i32) -> i32 {
    let mut sum = 0;
    
    for n in 0..range {
        let mut power = 
        sum += num::pow(2, 3);
        println!("{}", sum);
    }

    sum
}

fn main() {
    bin_2_dec(10);
}