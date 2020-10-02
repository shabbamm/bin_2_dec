# cool things i learned about rust today

## 10/2/2020

- there are two types of strings:
  - String
  - &str
- a string (&str) of characters can be sliced and flipped using '.chars()' and '.rev()'
- iterate from 0 to a number 'n' like 0..n
- step through each char of a sliced up string with '.next()'
- when comparing char's you can/sometimes have to wrap it with 'Some()' or 'None()' in order to do any meaningful boolean evals -> '==', '!=', etc.
- cast between number types with 'as' -> 2 as usize, 2.0 as i32, etc.
- returning a value from a rust function with the '-> \<T>' in the function definition
