#![allow(non_snake_case)] //for allowing snake case
#![allow(unused)] //for allowing unused variables or functions
use std::collections::HashMap;
use std::i32; //library fir i32
use std::io; //library for standard input output //library   for Hashmap
pub fn English(digits: u32) -> String //function which will return a string
{
    let mut wordstr = " ".to_string(); //Empty string
    let mut text: HashMap<u32, &str> = HashMap::new(); //defination of hashmap
                                                       // values
    text.insert(0, " "); //for 20,30,40 output
    text.insert(1, "One");
    text.insert(2, "Two");
    text.insert(3, "Three");
    text.insert(4, "Four");
    text.insert(5, "Five");
    text.insert(6, "Six");
    text.insert(7, "Seven");
    text.insert(8, "Eight");
    text.insert(9, "Nine");
    text.insert(10, "Ten");
    text.insert(11, "Eleven");
    text.insert(12, "Twelve");
    text.insert(13, "Thirteen");
    text.insert(14, "Forteen");
    text.insert(15, "Fifteen");
    text.insert(16, "Sixteen");
    text.insert(17, "Seventeen");
    text.insert(18, "Eighteen");
    text.insert(19, "Nineteen");
    text.insert(20, "Twenty");
    text.insert(30, "Thirty");
    text.insert(40, "Forty");
    text.insert(50, "Fifty");
    text.insert(60, "Sixty");
    text.insert(70, "Seventy");
    text.insert(80, "Eighty");
    text.insert(90, "Ninety");

    // println!("{}",text.len());

    wordstr = [text.get(&digits).unwrap().to_string()].join(" ");
    return wordstr; //string return
}

pub fn EnglishWords(int_currency: u32, Complete_string: &str) -> String {
    let mut string = " ".to_string();
    let mut x = int_currency;
    let mut currency_length = 0;
    while x != 0
    //Finding the length of the integer value
    {
        x /= 10;
        currency_length += 1;
    }
    let mut lengthcopy = currency_length;
    if currency_length > 9 {
        string =
            ["Number should have less than 9 digits before decimal point".to_string()].join(" ");
    }
    while currency_length != 0
    //spilts the digits from number
    {
        let mut lengthpow = u32::pow(10, currency_length - 1);
        let mut p = int_currency / lengthpow;
        let mut digits = p % 10;
        lengthpow = lengthpow / 10;
        currency_length -= 1;
        let mut digitcopy = digits;
        let mut digitcopy2: u32 = 0;
        if (currency_length == 1
            || currency_length == 4
            || currency_length == 6
            || currency_length == 8)
            && digits != 0
        //digits!=0 for 101 number bcaz if we dont take it it will simply print hundred
        {
            if digits == 1 {
                p = int_currency / lengthpow;
                digits = digits * 10 + p % 10;
                lengthpow = lengthpow / 10;
                currency_length -= 1;
            } else {
                digitcopy2 = digits * 10;
                p = int_currency / lengthpow;
                digits = p % 10;
                lengthpow = lengthpow / 10;
                currency_length -= 1;
                /*if digits==0
                {
                    currency_length=0;
                }*/
            }
        } //println!("{}",digits);
        if digitcopy == 0
        //if dont use this condition it will give result like 100001 One  Lakh  One Thousand
        {
            lengthcopy -= 1;
        }
        match lengthcopy {
            1..=2 => {
                if digitcopy != 0
                //digitcopy!=0   For 100,1000,10000 values
                {
                    if lengthcopy == 2
                    //for 2 digit numbers
                    {
                        string = [
                            string,
                            English(digitcopy2).to_string(),
                            English(digits).to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, English(digits).to_string()].join(" ");
                        lengthcopy -= 1
                    } //for single digit numbers
                }
            }
            3 => {
                if digitcopy != 0 {
                    string = [string, English(digits).to_string(), "Hundred".to_string()].join(" ");
                    lengthcopy -= 1
                }
            } //for 3 digit numbers
            4..=5 => {
                if digitcopy != 0 {
                    if lengthcopy == 5
                    //for 4 digit numbers
                    {
                        string = [
                            string,
                            English(digitcopy2).to_string(),
                            English(digits).to_string(),
                            "Thousand".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, English(digits).to_string(), "Thousand".to_string()].join(" ");
                        lengthcopy -= 1
                    }
                }
            } //for 5 digit numbers
            6..=7 => {
                if digitcopy != 0 {
                    if lengthcopy == 7
                    //for 7 digit numbers
                    {
                        string = [
                            string,
                            English(digitcopy2).to_string(),
                            English(digits).to_string(),
                            "Lakh".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, English(digits).to_string(), "Lakh".to_string()].join(" ");
                        lengthcopy -= 1
                    }
                }
            } //for 6 digit numbers
            8..=9 => {
                if digitcopy != 0 {
                    if lengthcopy == 9
                    //for 8 digit numbers
                    {
                        string = [
                            string,
                            English(digitcopy2).to_string(),
                            English(digits).to_string(),
                            "Crore".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, English(digits).to_string(), "Crore".to_string()].join(" ");
                        lengthcopy -= 1
                    }
                }
            } //for 9 digit numbers
            _ => print!(""),
        }
    }
    string = [string, Complete_string.to_string()].join(" ");
    //println!("{}",string);
    return string;
}