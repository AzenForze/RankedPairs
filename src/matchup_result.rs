use std::cmp::Ordering;
use std::fmt::{self, Formatter, Display};
use graph::Graph;

/**
The results of a matchup, including which candidate won, and how much they won by.
*/
#[derive(PartialEq, Eq)]
pub struct MatchupResult<'a>
{
    winner: &'a str,
    loser: &'a str,
    wins: &'a u32,
    loses: &'a u32,
    use_margin: bool
}

impl <'a> Ord for MatchupResult<'a>
{
    fn cmp(&self, other: &MatchupResult) -> Ordering
    {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> PartialOrd for MatchupResult<'a>
{
    fn partial_cmp(&self, other: &MatchupResult) -> Option<Ordering>
    {
        if self.use_margin
        {
            let margin = self.wins - self.loses;
            let other_margin = other.wins - other.loses;

            if margin < other_margin
            {
                return Some(Ordering::Less);
            }
            else if margin > other_margin
            {
                return Some(Ordering::Greater);
            }
            else {
                return Some(Ordering::Equal);
            }
        }
        else {
            if self.wins < other.wins
            {
                return Some(Ordering::Less);
            }
            else if self.wins > other.wins
            {
                return Some(Ordering::Greater);
            }
            else {
                return Some(Ordering::Equal);
            }
        }
    }
}


impl<'a> MatchupResult<'a>
{
    pub fn new(winner: &'a str, loser: &'a str, wins: &'a u32, loses: &'a u32, use_margin: bool) -> Self
    {
        MatchupResult{ winner: winner, loser: loser, wins: wins, loses: loses, use_margin: use_margin }
    }

    pub fn try_lock_in(&self, graph: &mut Graph<String>) -> bool
    {
        let is_path = graph.is_path(&self.winner.to_owned(), &self.loser.to_owned());

        if !is_path
        {
            graph.add_edge(self.loser.to_owned(), self.winner.to_owned());
            return true;
        }
        return false;
    }
}

impl<'a> Display for MatchupResult<'a>
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "{} wins vs {}; wv: {} margin: {}", self.winner, self.loser, self.wins, self.wins - self.loses)
    }
}
