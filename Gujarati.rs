#![allow(non_snake_case)] //for allowing snake case
#![allow(unused)] //for allowing unused variables or functions
use std::collections::HashMap;
use std::i32; //library fir i32
use std::io; //library for standard input output //library   for Hashmap


pub fn Gujarati(digits: u32) -> String {
    let mut text: HashMap<u32, &str> = HashMap::new();
    text.insert(0, " ");
    text.insert(1, "એક ");
    text.insert(2, "બે  ");
    text.insert(3, "ત્રણ ");
    text.insert(4, "ચાર ");
    text.insert(5, "પાંચ ");
    text.insert(6, "છ ");
    text.insert(7, "સાત ");
    text.insert(8, "આઠ  ");
    text.insert(9, "નવ ");
    text.insert(10, "દસ ");
    text.insert(11, "અગિયાર ");
    text.insert(12, "બાર ");
    text.insert(13, "તેર ");
    text.insert(14, "ચૌદ ");
    text.insert(15, "પંદર ");
    text.insert(16, "સોળ ");
    text.insert(17, "સત્તર ");
    text.insert(18, "અઢાર ");
    text.insert(19, "ઓગણિસ ");
    text.insert(20, "વીસ ");
    text.insert(21, "એકવીસ ");
    text.insert(22, "બાવીસ ");
    text.insert(23, "તેવીસ ");
    text.insert(24, "ચોવીસ ");
    text.insert(25, "પચ્ચીસ ");
    text.insert(26, "છવીસ ");
    text.insert(27, "સત્તાવીસ ");
    text.insert(28, "અઠ્ઠાવીસ");
    text.insert(29, "ઓગણત્રીસ");
    text.insert(30, "ત્રીસ");
    text.insert(31, "એકત્રીસ ");
    text.insert(32, "બત્રીસ ");
    text.insert(33, "તેત્રીસ ");
    text.insert(34, "ચોત્રીસ ");
    text.insert(35, "પાંત્રીસ ");
    text.insert(36, "છત્રીસ ");
    text.insert(37, "સડત્રીસ ");
    text.insert(38, "અડત્રીસ  ");
    text.insert(39, "ઓગણચાલીસ ");
    text.insert(40, "ચાલીસ ");
    text.insert(41, "એકતાલીસ ");
    text.insert(42, "બેતાલીસ	 ");
    text.insert(43, "ત્રેતાલીસ ");
    text.insert(44, "ચુંમાલીસ ");
    text.insert(45, "પિસ્તાલીસ ");
    text.insert(46, "છેતાલીસ ");
    text.insert(47, "સુડતાલીસ ");
    text.insert(48, "અડતાલીસ ");
    text.insert(49, "ઓગણપચાસ ");
    text.insert(50, "પચાસ	 ");
    text.insert(51, "એકાવન ");
    text.insert(52, "બાવન ");
    text.insert(53, "ત્રેપન ");
    text.insert(54, "ચોપન ");
    text.insert(55, "પંચાવન ");
    text.insert(56, "છપ્પન ");
    text.insert(57, "સત્તાવન ");
    text.insert(58, "અઠ્ઠાવન ");
    text.insert(59, "ઓગણસાઠ ");
    text.insert(60, "સાઈઠ ");
    text.insert(61, "એકસઠ ");
    text.insert(62, "બાસઠ");
    text.insert(63, "ત્રેસઠ ");
    text.insert(64, "ચોસઠ  ");
    text.insert(65, "પાંસઠ ");
    text.insert(66, "છાસઠ ");
    text.insert(67, "સડસઠ ");
    text.insert(68, "અડસઠ ");
    text.insert(69, "અગણોસિત્તેર ");
    text.insert(70, "સિત્તેર ");
    text.insert(71, "એકોતેર ");
    text.insert(72, "બોતેર ");
    text.insert(73, "તોતેર ");
    text.insert(74, "ચુમોતેર ");
    text.insert(75, "પંચોતેર ");
    text.insert(76, "છોતેર ");
    text.insert(77, "સિત્યોતેર ");
    text.insert(78, "ઇઠ્યોતેર ");
    text.insert(79, "ઓગણાએંસી ");
    text.insert(80, "એંસી ");
    text.insert(81, "એક્યાસી ");
    text.insert(82, "બ્યાસી ");
    text.insert(83, "ત્યાસી ");
    text.insert(84, "ચોર્યાસી ");
    text.insert(85, "પંચાસી ");
    text.insert(86, "છ્યાસી ");
    text.insert(87, "સિત્યાસી ");
    text.insert(88, "ઈઠ્યાસી ");
    text.insert(89, "નેવ્યાસી ");
    text.insert(90, "નેવું ");
    text.insert(91, "એકાણું ");
    text.insert(92, "બાણું ");
    text.insert(93, "ત્રાણું ");
    text.insert(94, "ચોરાણું ");
    text.insert(95, "પંચાણું ");
    text.insert(96, "છન્નું");
    text.insert(97, "સત્તાણું ");
    text.insert(98, "અઠ્ઠાણું ");
    text.insert(99, "નવ્વાણું ");
    text.insert(100, "સો ");
    let mut wordstr = String::new();
    wordstr.push_str(text.get(&digits).unwrap());
    return wordstr;
}

pub fn GujaratiWords(int_currency: u32, Complete_string: &str) -> String {
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
        string = ["શૂન્ય".to_string()].join("");
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
                            Gujarati(digits).to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string = [string, Gujarati(digits).to_string()].join(" ");
                        lengthcopy -= 1
                    } //for single digit numbers
                }
            }
            3 => {
                if digitcopy != 0 {
                    if int_currency == 100 {
                        digits = int_currency;
                        string = [string, Gujarati(digits).to_string()].join(" ");
                    } else {
                        string = [string, Gujarati(digits).to_string(), "સો".to_string()].join(" ");
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
                            Gujarati(digits).to_string(),
                            "હજાર".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, Gujarati(digits).to_string(), "હજાર".to_string()].join(" ");
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
                            Gujarati(digits).to_string(),
                            "લાખ".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, Gujarati(digits).to_string(), "લાખ".to_string()].join(" ");
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
                            Gujarati(digits).to_string(),
                            "કરોડ઼".to_string(),
                        ]
                        .join(" ");
                        lengthcopy -= 2
                    } else {
                        string =
                            [string, Gujarati(digits).to_string(), "કરોડ઼".to_string()].join(" ");
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
