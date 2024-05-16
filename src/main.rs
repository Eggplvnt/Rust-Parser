
use std::fs::File;
use std::io::Read;
use std::env;
use std::process;

// reads in user arguments for the file they want read, and whether they want the 
// program to output in scheme or prolog. Passes the file name and output type 
// to function that reads the file.
fn initial_input() {

    let args: Vec<String> = env::args().collect();

    let pass:String = String::from(&args[1]);

    let _program_type:String = String::from(&args[2]);

    read_file(pass, _program_type);
    
}

// checks with output argument is passed and prints its respective program output
// then it unwraps the text in the file into a strink and passes it into the Lexical
// analyzer that returns a vector of tokens. Passes vector of tokens into syntax 
// analyzer that returns vector of strings for scheme or prolog output. 
fn read_file(first_arg:String, second_arg:String) {

    if second_arg == "-s" {

        println!("; Processing input file {}", first_arg);

    } else if second_arg == "-p" {

        println!("/* processing input file {}", first_arg);

    }

    let mut file = File::open(first_arg).unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let tkns: Vec<String> = lex_anal(content);

    let output_create: Vec<String> = synt_anal(tkns, &second_arg);

    // checks which program argument was passed, and passes the vector of strings
    // to the function to print the correct output
    if second_arg == "-s" {

        println!("; Lexical and Syntax analysis passed");

        scheme_output(output_create);

    } else if second_arg == "-p" {

        println!("   Lexical and Syntax analysis passed */");

        prolog_output(output_create);

    }

}

// prints out the scheme program from the passed in file. Takes in a vector
// of strings.
fn scheme_output(scheme: Vec<String>) {

    for line in scheme {

        println!("{}", line);

    }


}

// prints out the prolog program from the passed in file. takes in a vector
// of strings.
fn prolog_output(prolog: Vec<String>) {

    println!("\nmain :-");
    
    let mut i = 0;

    while i < prolog.len() - 1 {

        println!("   {},", prolog[i]);

        i = i + 1;

    }

    println!("   {}.\n", prolog[prolog.len() - 1]);

    
}




// takes in a vector of tokens and the required output as a string. returns a 
// vector of strings for the required program output. Performs Syntax analysis
// on the tokens to make sure they follow the required grammar for a program. 
fn synt_anal(tkns:Vec<String>, code:&String) -> Vec<String> {

    // temporary vector to store lexemes from tokens
    let mut output: Vec<String> = vec![];

    // vector to store the strings that print out program in scheme or prolog
    let mut to_return: Vec<String> = vec![];

    // keeps track of which proccess program is in based on tokens
    let mut procc = 0; 

    // keeps track of how many variables may be required based on grammar 
    // and tokens
    let mut posit = 1;

    // string that stores the expected process
    let mut prog_expect = "";

    // string that stores the expected next token
    let mut expect = "";

    // stores which 'PROCESSOP' is being read to match correct # of ID's
    let mut which_rrmsc = "";

    // loop through tokens one by one, reads current token, and assigns 
    // which token is expected next, then compares expected token and current
    // if wrong, outputs error
    for (i, tkn) in tkns.iter().enumerate() {

        if tkn == "DATA" {

            procc = 1;

            expect = "COLON";

        } else if procc == 1 {

            if expect == "COLON" && tkns[i - 1] == "DATA" {

                if tkn.contains("COLON") {

                    expect = "ID";

                } else {

                    println!("Syntax Error: missing colon");
                    process::exit(1);

                }


            } else if expect == "ID" {

                if tkn.contains("ID") {

                    expect = "COLON";

                } else {

                    println!("Syntax Error: expected an ID, found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "COLON" {

                if tkn.contains("COLON") {

                    expect = "TYPE";

                } else {

                    println!("Syntax Error: expected \":\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "TYPE" {

                if tkn.contains("VECTOR") || tkn.contains("NUMBER") {

                    if tkns[i + 1] !=  "COMMA" {

                        prog_expect = &tkns[i + 1];

                        procc = 2;


                    } else {

                        expect = "COMMA";

                    }

                } else {

                    println!("Syntax Error: expected type (number | vector), found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "COMMA" {

                if tkn.contains("COMMA") {

                    expect = "ID";

                } else {

                    println!("Syntax Error: expected \",\", found {}", tkn);
                    process::exit(1);

                }

            }

        } else if (procc == 2) && (prog_expect == "INPUT") {

        
            if tkn.contains("INPUT") {

                if !tkn.contains("INPUT") {

                    println!("Syntax Error: expected \"input\", found {}", tkn);
                    process::exit(1);

                } else {

                    expect = "COLON";
                }

            } else if expect == "COLON" && tkns[i - 1] == "INPUT" {

                if tkn.contains("COLON") {

                    expect = "ID";

                } else {

                    println!("Syntax Error: missing colon");
                    process::exit(1);

                }

            } else if expect == "ID" {

                if tkn.contains("ID") {

                    expect = "ASSIGN";

                    output.push(tkn[3..tkn.len()].to_string());

                } else {

                    println!("Syntax Error: expected ID, found {}", tkn);
                    process::exit(1);

                }
                
            } else if expect == "ASSIGN" {

                if tkn.contains("ASSIGN") {

                    expect = "READ";

                } else {

                    println!("Syntax Error: expected \"=\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "READ" {

                if tkn.contains("READ") {

                    expect = "LPAREN";

                } else {

                    println!("Syntax Error: expected \"read\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "LPAREN" {

                if tkn.contains("LPAREN") {

                    expect = "STRING";

                } else {

                    println!("Syntax Error: expected \"(\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "STRING" {

                if tkn.contains("STRING") {

                    expect = "COMMA";

                    posit = 2;

                    output.push(tkn[8..tkn.len() - 1].to_string());

                } else {

                    println!("Syntax Error: expected string value, found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "COMMA" {

                if tkn.contains("COMMA") {

                    if posit == 2 {

                        expect = "BOOL";

                        posit = 3;

                    } else if posit == 3 {

                        expect = "NUM";

                        posit = 1;

                    } else if posit == 1 {

                        expect = "ID";

                    }

                } else {

                    println!("Syntax Error: expected \",\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "BOOL" {

                if tkn.contains("TRUE") || tkn.contains("FALSE") {

                    expect = "COMMA";

                    if tkn.contains("TRUE") {

                        if code == "-s" {   
                            output.push("#t".to_string());
                        } else {
                            output.push("true".to_string());
                        }

                    } else if tkn.contains("FALSE") {

                        if code == "-s" {
                            output.push("#f".to_string());
                        } else {
                            output.push("false".to_string());
                        }

                    }

                } else {

                    println!("Syntax Error: expected boolean value, found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "NUM" {

                if tkn.contains("NUM") {

                    expect = "RPAREN";

                    output.push(tkn[4..tkn.len()].to_string());

                } else {

                    println!("Syntax Error: expected integer value, found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "RPAREN" {

                if tkn.contains("RPAREN") {

                    if tkns[i + 1] != "COMMA" {

                        prog_expect = &tkns[i + 1];

                        procc = 3;

                    } else {

                        expect = "COMMA"

                    }

                } else {

                    println!("Syntax Error: expected \")\", found {}", tkn);
                    process::exit(1);

                }

            }

        } else if (procc == 3) && (prog_expect == "PROCESS") {

            if tkn.contains("PROCESS") {

                expect = "COLON";

            } else if expect == "COLON" {

                if tkn.contains(expect) {

                    expect = "ID";

                } else {

                    println!("Syntax Error: expected \":\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "ID" {

                if tkn.contains(expect) {

                    output.push(tkn[3..tkn.len()].to_string());

                    if posit == 1 {

                        expect = "ASSIGN";

                        posit = 2;

                    } else if posit == 2 {

                        if which_rrmsc == "TWO" {

                            expect = "COMMA";

                            posit = 3;

                        } else if which_rrmsc == "ONE" {

                            expect = "RPAREN";

                            posit = 1;

                        }

                    } else if posit == 3 {

                        expect = "RPAREN";

                        posit = 1;

                    }

                } else {

                    println!("Syntax Error: expected ID, found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "ASSIGN" {

                if tkn.contains("ASSIGN") {

                    expect = "RRMSC";

                } else {

                    println!("Syntax Error: expected \"=\", found {}", tkn);
                    process::exit(1);
                }

            } else if expect == "RRMSC" {

                output.push(tkn.to_string().to_lowercase());

                if tkn.contains("REGRESSIONA") || tkn.contains("REGRESSIONB") ||
                tkn.contains("CORRELATION") {

                    which_rrmsc = "TWO";

                } else if tkn.contains("STDDEV") || tkn.contains("MEAN") {

                    which_rrmsc = "ONE";

                } else {

                    println!("Syntax Error: expected calculation (regressiona | regressionb | mean | stddev | correlation), found {}", tkn);
                    process::exit(1);

                }

                expect = "LPAREN";

            } else if expect == "LPAREN" {

                if tkn.contains("LPAREN") {

                    expect = "ID";

                } else {

                    println!("Syntax Error: expected \"(\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "COMMA" {

                if tkn.contains("COMMA") {

                    expect = "ID";

                } else {

                    println!("Syntax Error: expected \",\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "RPAREN" {

                if tkn.contains("RPAREN") {

                    if tkns[i + 1] != "COMMA" {

                        prog_expect = &tkns[i + 1];

                        procc = 4;

                    } else {

                        expect = "COMMA";

                    }

                } else {

                    println!("Syntax Error: expected \")\", found {}", tkn);
                    process::exit(1);

                }

            }

        } else if (procc == 4) && (prog_expect == "OUTPUT") {

            if tkn.contains("OUTPUT") {

                expect = "COLON";

            } else if expect == "COLON" {

                if tkn.contains("COLON") {

                    expect = "STRING";

                } else {

                    println!("Syntax Error: expected \":\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "STRING" {

                output.push(tkn[7..tkn.len()].to_string());

                if tkn.contains("STRING") {

                    expect = "COMMA";

                    posit = 2;

                } else {

                    println!("Syntax Error: expected string value, found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "COMMA" {

                if tkn.contains("COMMA") {

                    if posit == 1 {

                        expect = "STRING";

                    } else if posit == 2{

                        expect = "ID";

                        posit = 1;

                    }

                } else {

                    println!("Syntax Error: expected \",\", found {}", tkn);
                    process::exit(1);

                }

            } else if expect == "ID" {

                output.push(tkn[3..tkn.len()].to_string());

                if tkn.contains("ID") {


                    if tkns[i + 1] != "COMMA" {

                        prog_expect = &tkns[i + 1];

                        procc = 5;

                    } else {

                        expect = "COMMA";

                    }

                } else {

                    println!("Syntax Error: expected ID, found {}", tkn);
                    process::exit(1);

                }

            }

        } else if (procc == 5) && (prog_expect == "END") {

            if tkn.contains("END") {

                expect = "PERIOD";

            } else if expect == "PERIOD" {

                if !tkn.contains("PERIOD") {

                    println!("Syntax Error: missing period");
                    process::exit(1);

                }

            }

        } else {

            println!("Syntax error: Program does not have the correct order");
            process::exit(1);

        }

        // creates a string that stores scheme or prolog code that is pushed into the vector 
        // that will be returned 
        let mut strn = "".to_string();

        if procc == 2 && output.len() == 4 {

            if code == "-s" {

                strn = format!("(define {} (read-csv \"{}\" {} {}))", output[0], output[1], output[2].to_lowercase(), output[3]);

            } else if code == "-p" {

                strn = format!("load_data_column('{}', {}, {}, V{})", output[1], output[2], output[3], output[0]);

            }
                // empties the vector to store new required tokens
                output = vec![];

                to_return.push(strn);

        } else if procc == 3 {

            if which_rrmsc == "ONE" && output.len() == 3 {

                if code == "-s" {

                    strn = format!("(define {} ({} {}))", output[0], output[1], output[2]);
                     
                } else if code == "-p" {

                    strn = format!("{}(V{}, V{})", output[1], output[2], output[0]);

                }

                output = vec![];

                to_return.push(strn);

            } else if output.len() == 4 {

                if code == "-s" {

                    strn = format!("(define {} ({} {} {}))", output[0], output[1], output[2], output[3]);

                } else if code == "-p" {

                    strn = format!("{}(V{}, V{}, V{})", output[1], output[2], output[3], output[0]);

                }

                output = vec![];

                to_return.push(strn);

            }

        } else if procc >= 4 && output.len() == 2 {

            if code == "-s" {    

                strn = format!("(display {})", output[0]);

                to_return.push(strn);
                to_return.push("(newline)".to_string());

                strn = format!("(display {})", output[1]);

                to_return.push(strn);
                to_return.push("(newline)".to_string());

            } else if code == "-p" {

                strn = format!("writeln({})", output[0]);

                to_return.push(strn);

                strn = format!("writeln(V{})", output[1]);

                to_return.push(strn);

            }

            output = vec![];

        }

    }

    // returns the vector of strings for scheme or prolog
    return to_return;
    
}

// performs a lexical analysis on the read in file by taking the file as a string
// argument. returns a vector of tokens. 
fn lex_anal(content:String) -> Vec<String> {

    // vector stores tokens as strings
    let mut tkns: Vec<String> = vec![];

    // vector of characters that compiles tokens
    let mut current: Vec<char> = vec![];

    // keeps track of whether a string is being read
    let mut qoute = 0;

    // splits the content into line by line
    for line in content.split("\n") {

        // reads the line, character by character
        for character in line.chars() {

            // adds character into temporary vector to match tokens
            if character.is_alphabetic() || character == '"' || qoute == 1 {

                if character.is_uppercase() {

                    println!("Lexical error was found, \"{}\" is supposed to be lowercase", character);
                    process::exit(1);

                } else {
                    
                    if character == '"' {

                        qoute += 1;

                    }
                    current.push(character);

                }

            } else if character.is_numeric() {

                tkns.push("NUM ".to_string() + &character.to_string());

            } else if qoute != 1 {

                // collects characters into string, empties vector, and compares 
                // the string to tokens that can be found in the grammar, if found
                // adds token to vector of tokens
                let fin: String = current.into_iter().collect();
                current = vec![];

                if fin == "data" {

                    tkns.push("DATA".to_string());

                } else if fin == "input" {

                    tkns.push("INPUT".to_string());

                } else if fin == "process" {

                    tkns.push("PROCESS".to_string())

                } else if fin == "output" {

                    tkns.push("OUTPUT".to_string());

                } else if fin == "end" {

                    tkns.push("END".to_string());

                } else if fin == "true" {

                    tkns.push("TRUE".to_string());

                } else if fin == "false" {

                    tkns.push("FALSE".to_string());

                } else if fin == "read" {

                    tkns.push("READ".to_string());

                } else if fin == "vector" {

                    tkns.push("VECTOR".to_string());

                } else if fin == "number" {

                    tkns.push("NUMBER".to_string());

                } else if fin == "regressiona" {

                    tkns.push("REGRESSIONA".to_string());

                } else if fin == "regressionb" {

                    tkns.push("REGRESSIONB".to_string());

                } else if fin == "mean" {

                    tkns.push("MEAN".to_string());

                } else if fin == "stddev" {

                    tkns.push("STDDEV".to_string());

                } else if fin == "correlation" {

                    tkns.push("CORRELATION".to_string());

                } else if fin.contains("\"") {

                    qoute = 0;
                    tkns.push("STRING ".to_string() + &fin);

                } else {
                    if !fin.is_empty() {
                        tkns.push("ID ".to_string() + &fin);
                    }

                }

                // checks to see if current character is a symbol that can be found
                // in the grammar
                if character == ':' {

                    tkns.push("COLON".to_string());

                } else if character == ',' {

                    tkns.push("COMMA".to_string());

                } else if character == '.' {

                    tkns.push("PERIOD".to_string());

                } else if character == '(' {

                    tkns.push("LPAREN".to_string());

                } else if character == ')' {

                    tkns.push("RPAREN".to_string());

                } else if character == '=' {

                    tkns.push("ASSIGN".to_string()) 

                } else {
                    if !character.is_whitespace() {
                        println!("Lexical error was found, \"{}\" is not a valid symbol", character);
                        process::exit(1);
                    }

                }

            }


        }

    }

    // returns a vector of tokens
    return tkns;

}

// runs the initial function to read in user arguments and start the program
fn main() {

    initial_input();

}

