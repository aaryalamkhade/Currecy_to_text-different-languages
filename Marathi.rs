#![allow(non_snake_case)] //for allowing snake case
#![allow(unused)] //for allowing unused variables or functions
use std::collections::HashMap;
use std::i32; //library fir i32
use std::io; //library for standard input output //library   for Hashmap

pub fn Marathi(digits: u32) -> String {
    let mut text: HashMap<u32, &str> = HashMap::new();
    text.insert(0, " ");
    text.insert(1, "एक ");
    text.insert(2, "दोन ");
    text.insert(3, "तीन ");
    text.insert(4, "चार ");
    text.insert(5, "पाच ");
    text.insert(6, "सहा ");
    text.insert(7, "सात ");
    text.insert(8, "आठ  ");
    text.insert(9, "नऊ ");
    text.insert(10, "दहा ");
    text.insert(11, "अकरा ");
    text.insert(12, "बारा ");
    text.insert(13, "तेरा ");
    text.insert(14, "चौदा ");
    text.insert(15, "पंधरा ");
    text.insert(16, "सोळा ");
    text.insert(17, "सतरा ");
    text.insert(18, "अठरा ");
    text.insert(19, "एकोणीस ");
    text.insert(20, "वीस ");
    text.insert(21, "एकवीस ");
    text.insert(22, "बावीस ");
    text.insert(23, "तेवीस ");
    text.insert(24, "चोवीस ");
    text.insert(25, "पंचवीस ");
    text.insert(26, "सव्वीस ");
    text.insert(27, "सत्तावीस ");
    text.insert(28, "अठ्ठावीस");
    text.insert(29, "एकोणतीस");
    text.insert(30, "तीस");
    text.insert(31, "एकतीस ");
    text.insert(32, "बत्तीस ");
    text.insert(33, "तेहेतीस ");
    text.insert(34, "चौतीस ");
    text.insert(35, "पस्तीस ");
    text.insert(36, "छत्तीस ");
    text.insert(37, "सदतीस ");
    text.insert(38, "अडतीस  ");
    text.insert(39, "एकोणचाळीस ");
    text.insert(40, "चाळीस ");
    text.insert(41, "एक्केचाळीस ");
    text.insert(42, "बेचाळीस ");
    text.insert(43, "त्रेचाळीस ");
    text.insert(44, "चव्वेचाळीस ");
    text.insert(45, "पंचेचाळीस ");
    text.insert(46, "सेहेचाळीस ");
    text.insert(47, "सत्तेचाळीस ");
    text.insert(48, "अठ्ठेचाळीस ");
    text.insert(49, "एकोणपन्नास ");
    text.insert(50, "पन्नास ");
    text.insert(51, "एक्कावन्न ");
    text.insert(52, "बावन्न ");
    text.insert(53, "त्रेपन्न ");
    text.insert(54, "चोपन्न ");
    text.insert(55, "पंचावन्न ");
    text.insert(56, "छप्पन्न ");
    text.insert(57, "सत्तावन्न ");
    text.insert(58, "अठ्ठावन्न ");
    text.insert(59, "एकोणसाठ ");
    text.insert(60, "साठ ");
    text.insert(61, "एकसष्ठ ");
    text.insert(62, "बासष्ठ");
    text.insert(63, "त्रेसष्ठ ");
    text.insert(64, "चौसष्ठ ");
    text.insert(65, "पासष्ठ ");
    text.insert(66, "सहासष्ठ ");
    text.insert(67, "सदुसष्ठ ");
    text.insert(68, "अडुसष्ठ ");
    text.insert(69, "एकोणसत्तर ");
    text.insert(70, "सत्तर ");
    text.insert(71, "एक्काहत्तर ");
    text.insert(72, "बाहत्तर ");
    text.insert(73, "त्र्याहत्तर ");
    text.insert(74, "चौरयाहत्तर ");
    text.insert(75, "पंच्याहत्तर ");
    text.insert(76, "शहात्तर ");
    text.insert(77, "सत्याहत्तर ");
    text.insert(78, "अठ्ठ्याहत्तर ");
    text.insert(79, "एकोणऐंशी ");
    text.insert(80, "ऐंशी ");
    text.insert(81, "एक्क्याऐंशी ");
    text.insert(82, "ब्याऐंशी ");
    text.insert(83, "त्र्याऐंशी ");
    text.insert(84, "चौऱ्याऐंशी ");
    text.insert(85, "पंच्याऐंशी ");
    text.insert(86, "शहाऐंशी ");
    text.insert(87, "सत्त्याऐंशी ");
    text.insert(88, "अठ्ठ्याऐंशी");
    text.insert(89, "एकोणनव्वद ");
    text.insert(90, "नव्वद ");
    text.insert(91, "एक्क्याण्णव");
    text.insert(92, "ब्याण्णव");
    text.insert(93, "त्र्याण्णव");
    text.insert(94, "चौऱ्याण्णव");
    text.insert(95, "पंच्याण्णव");
    text.insert(96, "शहाण्णव");
    text.insert(97, "सत्त्याण्णव");
    text.insert(98, "अठ्ठ्याण्णव");
    text.insert(99, "नव्व्याण्णव");
    text.insert(100, "शंभर");
    let mut wordstr = String::new();
    wordstr.push_str(text.get(&digits).unwrap());
    return wordstr;
}

pub fn MarathiWords(int_currency: u32, Complete_string: &str) -> String {
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
                            Marathi(digits).to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, Marathi(digits).to_string()].join(" ");
                        lengthcopy -= 1
                    } //for single digit numbers
                }
            }
            3 => {
                if digitcopy != 0 {
                    if int_currency == 100 {
                        digits = int_currency;
                        string = [string, Marathi(digits).to_string()].join(" ");
                    } else {
                        string = [string, Marathi(digits).to_string(), "शे".to_string()].join(" ");
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
                            Marathi(digits).to_string(),
                            "हजार".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, Marathi(digits).to_string(), "हजार".to_string()].join(" ");
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
                            Marathi(digits).to_string(),
                            "लाख".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, Marathi(digits).to_string(), "लाख".to_string()].join(" ");
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
                            Marathi(digits).to_string(),
                            "करोड".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, Marathi(digits).to_string(), "करोड".to_string()].join(" ");
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
