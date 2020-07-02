#![allow(non_snake_case)] //for allowing snake case
#![allow(unused)] //for allowing unused variables or functions
use std::collections::HashMap;
use std::i32; //library fir i32
use std::io; //library for standard input output //library   for Hashmap


pub fn Hindi(digits: u32) -> String {
    let mut text: HashMap<u32, &str> = HashMap::new();
    text.insert(0, " ");
    text.insert(1, "एक ");
    text.insert(2, "दो ");
    text.insert(3, "तीन ");
    text.insert(4, "चार ");
    text.insert(5, "पांच ");
    text.insert(6, "छह ");
    text.insert(7, "सात ");
    text.insert(8, "आठ  ");
    text.insert(9, "नौ ");
    text.insert(10, "दस ");
    text.insert(11, "ग्यारह ");
    text.insert(12, "बारह ");
    text.insert(13, "तेरह ");
    text.insert(14, "चौदह ");
    text.insert(15, "पंद्रह ");
    text.insert(16, "सोलह ");
    text.insert(17, "सत्रह ");
    text.insert(18, "अठारह ");
    text.insert(19, "उन्नीस ");
    text.insert(20, "बीस ");
    text.insert(21, "इकीस ");
    text.insert(22, "बाईस ");
    text.insert(23, "तेइस ");
    text.insert(24, "चौबीस ");
    text.insert(25, "पच्चीस ");
    text.insert(26, "छब्बीस ");
    text.insert(27, "सताइस ");
    text.insert(28, "अट्ठाइस");
    text.insert(29, "उनतीस");
    text.insert(30, "तीस");
    text.insert(31, "इकतीस ");
    text.insert(32, "बतीस ");
    text.insert(33, "तैंतीस ");
    text.insert(34, "चौंतीस ");
    text.insert(35, "पैंतीस ");
    text.insert(36, "छतीस ");
    text.insert(37, "सैंतीस ");
    text.insert(38, "अड़तीस  ");
    text.insert(39, "उनतालीस ");
    text.insert(40, "चालीस ");
    text.insert(41, "इकतालीस ");
    text.insert(42, "बयालीस	 ");
    text.insert(43, "तैतालीस ");
    text.insert(44, "चवालीस ");
    text.insert(45, "पैंतालीस ");
    text.insert(46, "छयालिस ");
    text.insert(47, "सैंतालीस ");
    text.insert(48, "अड़तालीस ");
    text.insert(49, "उनचास ");
    text.insert(50, "पचास ");
    text.insert(51, "इक्यावन ");
    text.insert(52, "बावन ");
    text.insert(53, "तिरपन ");
    text.insert(54, "चौवन ");
    text.insert(55, "पचपन ");
    text.insert(56, "छप्पन ");
    text.insert(57, "सतावन ");
    text.insert(58, "अठावन ");
    text.insert(59, "उनसठ ");
    text.insert(60, "साठ ");
    text.insert(61, "इकसठ ");
    text.insert(62, "बासठ");
    text.insert(63, "तिरसठ ");
    text.insert(64, "चौंसठ  ");
    text.insert(65, "पैंसठ ");
    text.insert(66, "छियासठ ");
    text.insert(67, "सड़सठ ");
    text.insert(68, "अड़सठ ");
    text.insert(69, "उनहतर ");
    text.insert(70, "सत्तर ");
    text.insert(71, "इकहतर ");
    text.insert(72, "बहतर ");
    text.insert(73, "तिहतर ");
    text.insert(74, "चौहतर ");
    text.insert(75, "पचहतर ");
    text.insert(76, "छिहतर ");
    text.insert(77, "सतहतर ");
    text.insert(78, "अठहतर ");
    text.insert(79, "उन्नासी ");
    text.insert(80, "अस्सी	 ");
    text.insert(81, "इक्यासी ");
    text.insert(82, "बयासी ");
    text.insert(83, "तिरासी ");
    text.insert(84, "चौरासी ");
    text.insert(85, "पचासी ");
    text.insert(86, "छियासी ");
    text.insert(87, "सतासी ");
    text.insert(88, "अट्ठासी");
    text.insert(89, "नवासी ");
    text.insert(90, "नब्बे ");
    text.insert(91, "इक्यानवे");
    text.insert(92, "बानवे");
    text.insert(93, "तिरानवे");
    text.insert(94, "चौरानवे");
    text.insert(95, "पचानवे");
    text.insert(96, "छियानवे");
    text.insert(97, "सतानवे");
    text.insert(98, "अट्ठानवे");
    text.insert(99, "निन्यानवे");
    text.insert(100, "एकसौ");
    let mut wordstr = String::new();
    wordstr.push_str(text.get(&digits).unwrap());
    return wordstr;
}

pub fn HindiWords(int_currency: u32, Complete_string: &str) -> String {
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
    if (int_currency) == 0 {
        string = ["शून्य".to_string()].join("");
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
        //let mut digitcopy2: u32 = 0;
        if (currency_length == 1
            || currency_length == 4
            || currency_length == 6
            || currency_length == 8)
            && digits != 0
        //digits!=0 for 101 number bcaz if we dont take it it will simply print hundred
        {
            p = int_currency / lengthpow;
            digits = digits * 10 + p % 10;
            lengthpow = lengthpow / 10;
            currency_length -= 1;

            /*if digits==0
            {
                currency_length=0;
            }*/
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
                            // Marathi(digitcopy2).to_string(),
                            Hindi(digits).to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, Hindi(digits).to_string()].join(" ");
                        lengthcopy -= 1
                    } //for single digit numbers
                }
            }
            3 => {
                if digitcopy != 0 {
                    if int_currency == 100 {
                        digits = int_currency;
                        string = [string, Hindi(digits).to_string()].join(" ");
                    } else {
                        string = [string, Hindi(digits).to_string(), "सौ".to_string()].join(" ");
                        lengthcopy -= 1
                    }
                }
            } //for 3 digit numbers
            4..=5 => {
                if digitcopy != 0 {
                    if lengthcopy == 5
                    //for 4 digit numbers
                    {
                        string = [
                            string,
                            //Marathi(digitcopy2).to_string(),
                            Hindi(digits).to_string(),
                            "हज़ार".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, Hindi(digits).to_string(), "हज़ार".to_string()].join(" ");
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
                            //Marathi(digitcopy2).to_string(),
                            Hindi(digits).to_string(),
                            "लाख".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, Hindi(digits).to_string(), "लाख".to_string()].join(" ");
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
                            // Marathi(digitcopy2).to_string(),
                            Hindi(digits).to_string(),
                            "करोड़".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, Hindi(digits).to_string(), "करोड़".to_string()].join(" ");
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