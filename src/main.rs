/*
use std::io;

fn main(){
  println!("what is your name; ");
  let mut name = String::new();

  io::stdin().read_line(&mut name)
    .expect("failed to receive input");
  
  println!("hi {} Nice 2 meet U", name.trim());
}

*/

use std::io;
use rand::Rng;


fn main(){
  let mut game_over:bool = false;
  let mut user_score:usize = 0;
  let mut computer_score:usize = 0;
  let choices:[&str; 3] = ["rock","paper","scissors"];

  while !game_over {
    println!("enter a choice [rock, paper, scissors] or quit to show results and quit");
    let mut user_choice:String = String::new();
    io::stdin().read_line(&mut user_choice)
      .expect("failed to receive input");
    
    // delete new lines spaces etc.... and convert it into lowercase
    user_choice = user_choice.trim().to_string();
    user_choice = user_choice.to_lowercase().to_string();

    // in case the user enter something else rather that rock paper scissors
    if choices.contains(&user_choice.as_str()){
      let computer_choice:String = choices[rand::thread_rng().gen_range(0..3)].to_string();
      // who is the winner ?
      let winner:String = who_wins(user_choice, computer_choice);

      match winner.as_str() {
        "user" => user_score += 1,
        "computer" => computer_score += 1,
        // dont care about this line
        _ => user_score = user_score,
      }

      if winner != "tie" {
        println!("winner {winner} | {winner}.score + 1");
      }else {
        println!("its a Tie!");
      }
    }

    else{
      if user_choice == "quit" {
        // see who is the clear winner by comparing the user score and the computer score
        let complete_winner:String = if user_score > computer_score {
          String::from("user")
        }else{
          if user_score == computer_score {
            String::from("No One!")
          }else{
            String::from("computer")
          }
        };

        // display the data like the score and the winner
        println!("computer-score = {computer_score} | user-score = {user_score}");
        println!("the clear winner is : {complete_winner}");

        // terminate the game becaus the user wants to quit
        game_over= true;
      }else{
        if user_choice.is_empty() {
          continue;
        }else{
          println!("invailed choice {user_choice}");
        }
      }
    }
  }

}


fn who_wins(user:String, computer:String) -> String {
  println!("\nuser {user} computer {computer}");
  if user == computer {
    return String::from("tie");
  }else{
    // there is a winner
    if (user == "rock" && computer == "scissors") || (user == "scissors" && computer == "paper") || (user == "paper" && computer == "rock" ){
      return String::from("user");
    }else{
      return String::from("computer");
    }
  }
}

