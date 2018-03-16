
use matchup::Matchup;
use table::{Values, Table};
use election::Election;


/// A table logging how many times each candidate defeats each other candidate.
/// Used for several Condorcet/Robin methods.
pub struct SumMatrix
{
    table: Table<String, String, Matchup>
}

impl SumMatrix
{
    pub fn new(election: &Election) -> Self
    {
        let mut sum_matrix = SumMatrix { table: Table::new() };

        for vote in election.votes()
        {
            sum_matrix.add_vote(vote);
        }

        return sum_matrix;
    }

    pub fn add_vote(&mut self, vote: &Vec<Vec<String>>)
    {
        for i in 0..vote.len()
        {
            for higher in &vote[i]
            {
                for j in (i+1)..vote.len()
                {
                    for lower in &vote[j]
                    {
                        self.add_win(higher, lower);
                    }
                }
            }
        }
    }

    fn add_win(&mut self, for_cand: &String, against_cand: &String)
    {
        // (A, B) and (B, A) should be treated as the same pair.
        let (a, b) = if for_cand < against_cand { (for_cand, against_cand) } else { (against_cand, for_cand) };


        self.table.entry(a, b).or_emplace(a, b).add_win_for(for_cand).unwrap()
    }

    
    /// Returns an iterator over the matchups.
    pub fn matchups(&self) -> Matchups
    {
        Matchups{ adapt: self.table.values() }
    }
}

use table::TableEntry;
use std::borrow::{Borrow, ToOwned};
use std::hash::Hash;

impl<'a, 'b, Q, U> TableEntry<'a, 'b, String, String, Q, U, Matchup>
    where
    String: Borrow<Q>,
    String: Borrow<U>,
    Q: ToOwned<Owned=String>+Hash+Eq+?Sized, 
    U: ToOwned<Owned=String>+Hash+Eq+?Sized
{
    fn or_emplace<S, T>(self, cand1: &S, cand2: &T) -> &'a mut Matchup
        where
        String: Borrow<S>,
        String: Borrow<T>,
        S: ToOwned<Owned=String>+?Sized,
        T: ToOwned<Owned=String>+?Sized
    {
        match self
        {
            TableEntry::Vacant(entry) => {
                entry.insert(Matchup::new(cand1.to_owned(), cand2.to_owned()))
            },
            TableEntry::Occupied(entry) => { entry.get() }
        }
    }
}


/// An iterator over the matrix's matchups
pub struct Matchups<'a>
{
    adapt: Values<'a, String, String, Matchup>
}

impl<'a> Iterator for Matchups<'a>
{
    type Item = &'a Matchup;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.adapt.next()
    }
}
