use clap::Parser;

const SUBS_I : &str =
"àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžź
ż";
const SUBS_O: &str =
"aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzz
z";

fn conv(c:char) -> char {
    // for index in SUBS_I.chars().enumerate().filter(|(_, ch)| *ch == c).map(|(i, _)| i) {
    //     println!("Index: {}", index);
    // }
    // println!("{}", SUBS_I.chars().enumerate().filter(|(_, ch)| *ch == c).map(|(i, _)| i).next());
    match SUBS_I.chars().enumerate().filter(|(_, ch)| *ch == c).map(|(i, _)| i).next() {
        Some(i) => match SUBS_O.chars().nth(i) {
            Some(i) => return i,
            None => return c
        },
        None => return c,
    }
}

fn slugify(s: &str) -> String{
    // ● tutti i caratteri accentati riconosciuti vengono convertiti nell’equivalente non accentato
    // ● tutto viene convertito in minuscolo
    // ● ogni altro carattere rimanente che non sia in [a-z][0-9] viene convertito in “-”
    // ● due “-” consecutivi non sono ammessi, solo il primo viene tenuto
    // ● un “-” finale non è ammesso a meno che non sia l’unico carattere nella stringa
    let mut res = String::new(); 
    let mut flag = true;
    for c in s.chars(){
        if c.is_alphanumeric(){
            if c.is_alphabetic(){
                res.push(conv(c.to_ascii_lowercase()));
            }
            else {
                res.push(c);
            }
            flag = true;
        }
        else {
            if flag {
                res.push_str("-");
                flag = false;
            }
        }
    }
    if res.ends_with('-') && res.len()>1{
        res.pop();
    }
    res
}

fn splitter(s: &str) -> Result<Vec<String>, &str>{
    match true {
        true => Ok(s.split_whitespace().map(|v| v.to_string()).collect()), 
        false => Err("stocazzo"),
    }

    
}

#[derive(Parser, Debug)]
struct Args {
 // input string
    #[arg(short, long, value_parser = clap::builder::ValueParser::new(splitter))]
    slug_in: Vec<String>,
    #[arg(short, long, value_parser = str::parse::<i32>)]
    repeat: i32,
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    for s in args.slug_in{
        println!("{}", slugify(&s));
    }
    println!("Repeat: {} - Verbose: {}", args.repeat, args.verbose);
}

#[cfg(test)]
mod tests {
    use super::*;
}

// 1
#[test]
fn letter_with_accent() {
    // valore = preparazione test
    assert_eq!(slugify("à"), "a")
}

// 2
#[test]
fn letter_without_accent() {
    // valore = preparazione test
    assert_eq!(slugify("A"), "a")
}

// 3
#[test]
fn unknown_prohibited_letter() {
    // valore = preparazione test
    assert_eq!(slugify("ひ"), "ひ")
}

// 4
#[test]
fn unknown_accented_letter() {
    // valore = preparazione test
    assert_eq!(slugify("ῶ"), "ῶ")
}

// 5
#[test]
fn string_wit_more_words_separated_by_space() {
    // valore = preparazione test
    assert_eq!(slugify("Ciao coglione"), "ciao-coglione")
}

// 6
#[test]
fn string_with_multiple_accented_letter() {
    // valore = preparazione test
    assert_eq!(slugify("ŕÿ ffsf!"), "ry-ffsf")
}

// 7
#[test]
fn empty_string() {
    // valore = preparazione test
    assert_eq!(slugify(""), "")
}

// 8
#[test]
fn string_with_multiple_consecutive_spaces() {
    // valore = preparazione test
    assert_eq!(slugify("Ciao      coglione"), "ciao-coglione")
}

// 9
#[test]
fn string_with_multiple_prohibited_characters_in_a_row() {
    // valore = preparazione test
    assert_eq!(slugify("ciao!!ciao"), "ciao-ciao")
}

// 10
#[test]
fn string_with_only_prohibited_chars() {
    // valore = preparazione test
    assert_eq!(slugify("!!!!!!"), "-")
}

// 11
#[test]
fn string_that_ends_with_space() {
    // valore = preparazione test
    assert_eq!(slugify("ciao "), "ciao")
}

// 12
#[test]
fn string_with_multiple_prohibited_char_at_the_end() {
    // valore = preparazione test
    assert_eq!(slugify("ciao!-!-"), "ciao")
}
