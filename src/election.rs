
pub struct Election
{
    pub ballots: Vec<Vec<Vec<String>>>
}

impl Election
{
    pub fn new() -> Self
    {
        Election {ballots: Vec::new()}
    }

    pub fn votes(&self) -> &[Vec<Vec<String>>]
    {
        return &self.ballots[..];
    }
    

    /// Parses a String vote.
    /// "A>B=C>D" creates [[a], [b, c], [d]]
    pub fn add_vote(&mut self, data: &str)
    {
        let mut ballot: Vec<Vec<String>> = Vec::new();

        for rank in data.split('>')
        {
            let mut ranks: Vec<String> = Vec::new();

            for c in rank.split('=')
            {
                ranks.push(c.to_owned());
            }
            ballot.push(ranks);
        }

        self.ballots.push(ballot);
    }

    /// Parses a String into ballots.
    /// Format: Per line:
    /// [amount]:[vote]
    /// (See self.add_vote() for [vote])
    pub fn add_ballots(&mut self, data: &str) -> Result<(), ElectionParseError>
    {
        for line in data.trim().lines()
        {
            let mut parts = line.trim().split(':');

            let amount_str = match parts.next()
            {
                Some(amount_str) => {
                    if amount_str.len() == 0 {
                        return Err(ElectionParseError::NoAmountError)
                    } else {
                        amount_str
                    }
                },
                None => return Err(ElectionParseError::NoAmountError)
            };
            
            let vote = match parts.next()
            {
                Some(vote) => vote,
                None => return Err(ElectionParseError::NoVoteError)
            };

            if let Some(_) = parts.next()
            {
                return Err(ElectionParseError::ExcessDataError);
            }
            
            let amount = match amount_str.parse::<u32>()
            {
                Ok(amount) => amount,
                Err(e) => return Err(ElectionParseError::AmountParseError(e))
            };

            for _ in 0..amount
            {
                self.add_vote(vote);
            }
        }

        return Ok(());
    }
}

use std::num::ParseIntError;

#[derive(Debug)]
pub enum ElectionParseError
{
    NoAmountError,
    NoVoteError,
    ExcessDataError,
    AmountParseError(ParseIntError)
}

use std::fmt;
impl fmt::Display for ElectionParseError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.description())
    }
}

use std::error::Error;

impl Error for ElectionParseError
{
    fn description(&self) -> &str
    {
        match *self
        {
            ElectionParseError::NoAmountError => "could not find amount",
            ElectionParseError::NoVoteError => "could not find vote",
            ElectionParseError::ExcessDataError => "too many sections",
            ElectionParseError::AmountParseError(ref e) => e.description()
        }
    }
}