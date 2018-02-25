
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

    pub fn result(&self, use_margin: bool) -> MatchupResult
    {
        if self.first_wins > self.second_wins
        {
            MatchupResult::new(&self.first, &self.second, &self.first_wins, &self.second_wins, use_margin)
        }
        else
        {
            MatchupResult::new(&self.second, &self.first, &self.second_wins, &self.first_wins, use_margin)
        }
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
