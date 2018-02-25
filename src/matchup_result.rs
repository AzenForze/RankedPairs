use std::cmp::Ordering;
use std::fmt::{self, Formatter, Display};
use graph::Graph;

/**
The results of a matchup, including which one won, and how much they won by.
*/
pub struct MatchupResult
{
    winner: String,
    loser: String,
    wins: u32,
    margin: u32
}

impl MatchupResult
{
    pub fn new(winner: String, loser: String, wins: u32, margin: u32) -> Self
    {
        MatchupResult { winner: winner, loser: loser, wins:wins, margin: margin }
    }

    pub fn cmp(&self, other: &MatchupResult, use_margin: bool) -> Ordering
    {
        if use_margin
        {
            if self.margin < other.margin
            {
                return Ordering::Less;
            }
            else if self.margin > other.margin
            {
                return Ordering::Greater;
            }
            else {
                return Ordering::Equal;
            }
        }
        else {
            if self.wins < other.wins
            {
                return Ordering::Less;
            }
            else if self.wins > other.wins
            {
                return Ordering::Greater;
            }
            else {
                return Ordering::Equal;
            }
        }
    }

    pub fn try_lock_in(&self, graph: &mut Graph<String>) -> bool
    {
        let is_path = graph.is_path(&self.winner, &self.loser);

        if !is_path
        {
            graph.add_edge(self.loser.clone(), self.winner.clone());
            return true;
        }
        return false;
    }
}

impl Display for MatchupResult
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "{} wins vs {}; wv: {} margin: {}", self.winner, self.loser, self.wins, self.margin)
    }
}
