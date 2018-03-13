
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
            for i in 0..vote.len()
            {
                for higher in &vote[i]
                {
                    for j in (i+1)..vote.len()
                    {
                        for lower in &vote[j]
                        {
                            // (A, B) and (B, A) should be treated as the same matchup.
                            let (A, B) = if higher < lower { (higher, lower) } else { (lower, higher) };

                            // Could use entry pattern, but it would make a lot of unecessary copies of keys.
                            let mut new = false;

                            match sum_matrix.get_mut(&A, &B)
                            {
                                Some(matchup) => { matchup.add_win_for(&higher).unwrap(); }
                                None => { new = true; }
                            }
                            
                            if new
                            {
                                let mut new_matchup = Matchup::new(A.clone(), B.clone());
                                new_matchup.add_win_for(&higher).unwrap();
                                sum_matrix.insert(A.clone(), B.clone(), new_matchup);
                            }
                        }
                    }
                }
            }
        }

        /*
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
                            let (first, second) = if candidate < other_candidate { (candidate, other_candidate) } else { (other_candidate, candidate) } ;

                            /* Could use entry patern, but they will often be ocupied, no need to copy keys. */
                            let mut new = false;

                            match sum_matrix.get_mut(&first, &second)
                            {
                                Some(matchup) => { matchup.add_win_for(&candidate).unwrap(); }

                                None => { new = true; }
                            }
                            
                            if new
                            {
                                let mut new_matchup = Matchup::new(first.clone(), second.clone());
                                new_matchup.add_win_for(&candidate).unwrap();
                                sum_matrix.insert(first.clone(), second.clone(), new_matchup);
                            }
                        }
                    }
                }
            }
        }
        */

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
