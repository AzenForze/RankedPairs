
use matchup::Matchup;
use table::{Values, Table};

/**
A table logging how many times each candidate defeats each other candidate.
Used for several Condorcet/IRR methods.
*/
pub struct SumMatrix
{
    table: Table<String, String, Matchup>
}

impl SumMatrix
{
    /**
    for each candidate A, finds each candidate from a later rank B, and adds one to A's victories over B.
    */
    pub fn new(ballots: &Vec<Vec<Vec<String>>>) -> Self
    {
        let mut sum_matrix: Table<String, String, Matchup> = Table::new();
        
        for vote in ballots
        {
            for (i, rank) in vote.iter().enumerate()
            {
                for candidate in rank
                {
                    for other_rank in vote.iter().skip(i+1)
                    {
                        for other_candidate in other_rank
                        {
                            /* Could use entry patern, but they will often be ocupied, no need to copy keys. */
                            let mut new = false;

                            match sum_matrix.get_mut(&candidate, &other_candidate)
                            {
                                Some(matchup) => { matchup.add_win_for(&candidate).unwrap(); }

                                None => { new = true; }
                            }
                            
                            if new
                            {
                                let mut new_matchup = Matchup::new(candidate.clone(), other_candidate.clone());
                                new_matchup.add_win_for(&candidate).unwrap();
                                sum_matrix.insert(candidate.clone(), other_candidate.clone(), new_matchup);
                            }
                        }
                    }
                }
            }
        }

        return SumMatrix { table: sum_matrix };
    }

    /**
    Returns an iterator the matchups.
    */
    pub fn matchups(&self) -> Matchups
    {
        Matchups{ adapt: self.table.values() }
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
