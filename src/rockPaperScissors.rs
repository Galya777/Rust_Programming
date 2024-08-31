#[derive(Debug)]
pub enum State {
    Rock = 1,
    Scissors = 2,
    Paper = 3,
    Unknown

}
use std::io;
use rand::Rng;
fn main() {
    println!("Choose your item: ");
    println!("1.Rock ");
    println!("2.Scissors ");
    println!("3.Paper ");
    let _choiceUser: State;
    let _choiceComputer: State;
    //user input
    let mut input = String::new();

    println!("Enter an integer:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut _userInput: i8 = input.trim().parse().expect("Invalid input");

    //check options
    while _userInput <= 0 || _userInput > 3 {
        println!("Invalid input! Try again");
        _userInput = input.trim().parse().expect("Invalid input");
    }
    if _userInput == 1 {
        _choiceUser = State::Rock;
    } else if _userInput == 3{
        _choiceUser = State::Paper;
    } else if _userInput == 2 {
        _choiceUser = State::Scissors;
    }else{
        _choiceUser=State::Unknown;
    }

    //cpmputrer choises
    let mut rng = rand::thread_rng();
    // Generate random number in the range [1,3]
    let mut num: i8 = rand::thread_rng().gen_range(1..4);
    if num == 1 {
        _choiceComputer = State::Rock;
    } else if num == 3{
        _choiceComputer = State::Paper;
    } else if num == 2 {
        _choiceComputer = State::Scissors;
    }else{
        _choiceComputer=State::Unknown;
    }
    //calculate winner
    let winner: &str="";
    let mut draft: bool = false;
    //Rules: rock beats scissors, scissors beats paper, paper beats rock
    //1 beats 2, 2 beats 3, 3 beats 1
    if _userInput == num {
        draft = true;
    }
    println!("{:?}", _choiceUser);
    println!("{:?}", _choiceComputer);
    //print winner
    if draft {
        println!("Draft!");
    } else {
        println!("{ } won!", winner);
    }
}
