use graph::Graph;
use std::fmt::{self, Formatter, Display};

/**
Data about a matchup between two candidates.
*/
pub struct Matchup
{
    winner: String,
    winning_votes: u32,
    loser: String,
    losing_votes: u32
}

impl Matchup
{
    pub fn new(c1: String, c1_wins: u32, c2: String, c2_wins: u32) -> Self
    {
        if c1_wins > c2_wins
        {
            return Matchup {
                winner: c1, 
                winning_votes: c1_wins, 
                loser: c2, 
                losing_votes: c2_wins };
        }
        else
        {
            return Matchup {
                winner: c2,
                winning_votes: c2_wins,
                loser: c1,
                losing_votes: c1_wins }
        }

    }

    pub fn margin(&self) -> u32
    {
        return self.winning_votes - self.losing_votes;
    }

    pub fn winning_votes(&self) -> u32
    {
        return self.winning_votes;
    }

    /**
    Check if it's possible to go from winner->loser.
    If it is NOT possible, create a path from the loser->winner (don't create a cycle).
    */
    pub fn lock_in(&self, graph: &mut Graph) -> bool
    {
        if !graph.is_path(&self.winner, &self.loser)
        {
            graph.add_edge(self.loser.clone(), self.winner.clone());
            return true;
        }
        return false;
    }
}

impl Display for Matchup
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        
        write!(f, "{} ({}) vs {} ({}): {}", self.winner, self.winning_votes, self.loser, self.losing_votes, self.margin())
    }

}
