
use matchup_result::MatchupResult;

/**
A matchup between two candidates, including how many points each has versus the other.
*/
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

    pub fn result(&self) -> MatchupResult
    {
        let winner: &str;
        let loser: &str;
        let wins: u32;
        let margin: u32;
        
        if self.first_wins > self.second_wins
        {
            winner = &self.first;
            loser = &self.second;
            wins = self.first_wins;
            margin = self.first_wins - self.second_wins;
        }
        else
        {
            winner = &self.second;
            loser = &self.first;
            wins = self.second_wins;
            margin = self.second_wins - self.first_wins;
        }
        
        MatchupResult::new(winner, loser, wins, margin)
    }

    pub fn add_win_for(&mut self, winner: &String) -> Option<String>
    {
        if *winner == self.first
        {
            self.first_wins += 1;
            return None;
        }
        else if *winner == self.second
        {
            self.second_wins += 1;
            return None;
        }
        else {
            return Some("Winner wasn't in matchup".to_string());
        }
    }

}
