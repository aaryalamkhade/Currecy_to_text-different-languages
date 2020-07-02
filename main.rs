#![allow(non_snake_case)] //for allowing snake case
#![allow(unused)] //for allowing unused variables or functions
mod Bengoli;mod English;mod Gujarati;mod Hindi;mod Marathi;
use std::collections::HashMap;
use std::i32; //library fir i32
use std::io; //library for standard input output //library   for Hashmap

fn main() {
    println!("............Currency_To_Text.............");
    println!("Enter 1 for English conversion\nEnter 2 for Marathi conversion\nEnter 3 for Hindi conversion\nEnter 4 for Gujarati conversion\nEnter 5 for Bengoli conversion  ");
    //let mut loopval=String::new();
    //io::stdin().read_line(&mut loopval).expect(("Fail to read line"));
    //let loopvalcopy:i32=loopval.trim().parse().ok().expect("Program can only process numbers");

    let mut Choice = String::new();
    io::stdin()
        .read_line(&mut Choice)
        .expect("Fail to read Line");
    println!("Enter your Currency:");
    let choiceagain: i32 = Choice
        .trim()
        .parse()
        .ok()
        .expect("program can only process numbers");
    let mut currency = String::new();
    let mut Complete_string = " ".to_string();
    io::stdin()
        .read_line(&mut currency)
        .expect("Fail to read Line");
    let float_currency: f64 = currency
        .trim()
        .parse()
        .ok()
        .expect("program can only process numbers");
    let int_currency = float_currency as u32;
    let _diff: f64 = float_currency - f64::from(int_currency);
    let _fn: f64 = _diff * f64::from(100);
    //finding float value
    let _int_fn = _fn.round(); //round function
    let _intval = _int_fn as u32;
    if _intval != 0 {
        //decimal Number addition to the string
        let mut _decimalNo_pow = u32::pow(10, 2);
        //str2 = [str2,_decimalNo.to_string(),"/".to_string(), _decimalNo_pow.to_string()].join(" ");
        Complete_string = [
            _intval.to_string(),
            "/".to_string(),
            _decimalNo_pow.to_string(),
        ]
        .join(" "); //Decimal number printing
    }
    //println!("{}",Complete_string);
match choiceagain {
        1 => println!("{}", English::EnglishWords(int_currency, &Complete_string)),
        2 => println!("{}", Marathi::MarathiWords(int_currency, &Complete_string)),
        3 => println!("{}", Hindi::HindiWords(int_currency, &Complete_string)),
        4 => println!("{}",Gujarati::GujaratiWords(int_currency, &Complete_string)),
        5 => println!("{}", Bengoli::BengoliWords(int_currency, &Complete_string)),
        _ => println!("Wrong choice:"),
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(English::EnglishWords(1, " "), "  One  ")
    }
    #[test]
    fn test2() {
        assert_eq!(English::EnglishWords(12, "23 / 100"), "    Twelve 23 / 100")
    }
    #[test]
    fn test3() {
        assert_eq!(
            English::EnglishWords(152, "88 / 100"),
            "  One Hundred Fifty Two 88 / 100"
        )
    }
    #[test]
    fn test4() {
        assert_eq!(
            English::EnglishWords(1000, "23 / 100"),
            "  One Thousand 23 / 100"
        )
    }
    #[test]
    fn test5() {
        assert_eq!(
            English::EnglishWords(1234587, "16 / 100"),
            "    Twelve Lakh Thirty Four Thousand Five Hundred Eighty Seven 16 / 100"
        )
    }
    #[test]
    fn test6() {
        assert_eq!(
            English::EnglishWords(100000002, "29 / 100"),
            "    Ten Crore Two 29 / 100"
        )
    }
    #[test]
    fn test7() {
        assert_eq!(Marathi::MarathiWords(1, " "), "  एक   ")
    }
    #[test]
    fn test8() {
        assert_eq!(Marathi::MarathiWords(12, "23 / 100"), "  बारा  23 / 100")
    }
    #[test]
    fn test9() {
        assert_eq!(
            Marathi::MarathiWords(152, "88 / 100"),
            "  एक  शे बावन्न  88 / 100"
        )
    }
    #[test]
    fn test10() {
        assert_eq!(
            Marathi::MarathiWords(1000, "23 / 100"),
            "  एक  हजार 23 / 100"
        )
    }
    #[test]
    fn test11() {
        assert_eq!(
            Marathi::MarathiWords(1234587, "16 / 100"),
            "  बारा  लाख चौतीस  हजार पाच  शे सत्त्याऐंशी  16 / 100"
        )
    }
    #[test]
    fn test12() {
        assert_eq!(
            Marathi::MarathiWords(100000002, "29 / 100"),
            "  दहा  करोड दोन  29 / 100"
        )
    }

    //hindi testing
    #[test]
    fn test13() {
        assert_eq!(Hindi::HindiWords(1, " "), "  एक   ")
    }
    #[test]
    fn test14() {
        assert_eq!(Hindi::HindiWords(12, "23 / 100"), "  बारह  23 / 100")
    }
    #[test]
    fn test15() {
        assert_eq!(
            Hindi::HindiWords(152, "88 / 100"),
            "  एक  सौ बावन  88 / 100"
        )
    }
    #[test]
    fn test16() {
        assert_eq!(Hindi::HindiWords(1000, "23 / 100"), "  एक  हज़ार 23 / 100")
    }
    #[test]
    fn test17() {
        assert_eq!(
            Hindi::HindiWords(1234587, "16 / 100"),
            "  बारह  लाख चौंतीस  हज़ार पांच  सौ सतासी  16 / 100"
        )
    }
    #[test]
    fn test18() {
        assert_eq!(
            Hindi::HindiWords(100000002, "29 / 100"),
            "  दस  करोड़ दो  29 / 100"
        )
    }

    //Gujarati testing
    #[test]
    fn test19() {
        assert_eq!(Gujarati::GujaratiWords(1, " "), "  એક   ")
    }
    #[test]
    fn test20() {
        assert_eq!(Gujarati::GujaratiWords(12, "23 / 100"), "  બાર  23 / 100")
    }
    #[test]
    fn test21() {
        assert_eq!(
            Gujarati::GujaratiWords(152, "88 / 100"),
            "  એક  સો બાવન  88 / 100"
        )
    }
    #[test]
    fn test22() {
        assert_eq!(
            Gujarati::GujaratiWords(1000, "23 / 100"),
            "  એક  હજાર 23 / 100"
        )
    }
    #[test]
    fn test23() {
        assert_eq!(
            Gujarati::GujaratiWords(1234587, "16 / 100"),
            "  બાર  લાખ ચોત્રીસ  હજાર પાંચ  સો સિત્યાસી  16 / 100"
        )
    }
    #[test]
    fn test24() {
        assert_eq!(
            Gujarati::GujaratiWords(100000002, "29 / 100"),
            "  દસ  કરોડ઼ બે   29 / 100"
        )
    }

    //Bengolitesting
    #[test]
    fn test25() {
        assert_eq!(Bengoli::BengoliWords(1, " "), "  এক   ")
    }
    #[test]
    fn test26() {
        assert_eq!(Bengoli::BengoliWords(12, "23 / 100"), "  বারো  23 / 100")
    }
    #[test]
    fn test27() {
        assert_eq!(
            Bengoli::BengoliWords(152, "88 / 100"),
            "  এক  শো বাহান্নো  88 / 100"
        )
    }
    #[test]
    fn test28() {
        assert_eq!(
            Bengoli::BengoliWords(1000, "23 / 100"),
            "  এক  হাজার  23 / 100"
        )
    }
    #[test]
    fn test29() {
        assert_eq!(
            Bengoli::BengoliWords(1234587, "16 / 100"),
            "  বারো  লাখ চৌত্রিশ  হাজার  পাঁচ  শো সাতাশি  16 / 100"
        )
    }
    #[test]
    fn test30() {
        assert_eq!(
            Bengoli::BengoliWords(100000002, "29 / 100"),
            "  দশ  কোটি  দুই  29 / 100"
        )
    }
}


