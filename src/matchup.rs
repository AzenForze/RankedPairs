
use matchup_result::MatchupResult;

/**
A matchup between two candidates, including how many points each has versus the other.
*/
#[derive(Debug, Clone)]
pub struct Matchup
{
    first: String,
    second: String,
    first_wins: u32,
    second_wins: u32
}

impl Matchup
{
    pub fn new(first: String, second: String) -> Self
    {
        Matchup { first: first, second: second, first_wins: 0, second_wins: 0 }
    }
    
    pub fn result<T>(&self, use_margin: T) -> MatchupResult where T: Into<bool>
    {
        let (winner, loser, wins, loses) = if self.first_wins > self.second_wins
        {
            (&self.first, &self.second, &self.first_wins, &self.second_wins)
        }
        else
        {
            (&self.second, &self.first, &self.second_wins, &self.first_wins)
        };

        MatchupResult::new(winner, loser, wins, loses, use_margin.into())
    }

    pub fn add_win_for(&mut self, winner: &str) -> Result<(), MatchupError>
    {
        if *winner == self.first
        {
            self.first_wins += 1;
            return Ok(());
        }
        else if *winner == self.second
        {
            self.second_wins += 1;
            return Ok(());
        }
        else {
            return Err(MatchupError::new("Winner wasn't in matchup".to_owned()));
        }
    }
}

use std::error::Error;

#[derive(Debug)]
pub struct MatchupError
{
    description: String
}

impl MatchupError
{
    fn new(description: String) -> Self
    {
        MatchupError{ description: description }
    }
}

impl Error for MatchupError
{
    fn description(&self) -> &str
    {
        return &self.description;
    }
}

use std::fmt::{Formatter, Display, self};

impl Display for MatchupError
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "{}", self.description)
    }
}
