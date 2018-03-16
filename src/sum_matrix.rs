
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
    fn test(&mut self)
    {
        self.table.get_mut("row", "column");
    }

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
        
        self.entry(a, b).or_create().add_win_for(for_cand).unwrap()
    }

    
    /// Returns an iterator over the matchups.
    pub fn matchups(&self) -> Matchups
    {
        Matchups{ adapt: self.table.values() }
    }


    /// entry return an entry that takes references as keys which can be cloned if they need to be inserted.
    fn entry<'a, 'b, 'c>(&'a mut self, row: &'b String, column: &'c String) -> EntryClone<'a, 'b, 'c>
    {
        if self.table.contains(row, column)
        {
            return EntryClone::Occupied(OccupiedEntryClone { reference: self, row: row, column: column } );
        }
        else {
            return EntryClone::Vacant(VacantEntryClone { reference: self, row: row, column: column } );
        }
    }
}














/// An entry that takes references as keys which can be cloned if they need to be inserted.
enum EntryClone<'a, 'b, 'c>
{
    Vacant(VacantEntryClone<'a, 'b, 'c>),
    Occupied(OccupiedEntryClone<'a, 'b, 'c>)
}

impl<'a, 'b, 'c> EntryClone<'a, 'b, 'c>
{
    /*
    /// Returns the pair's matchup, or insers and returns the provided matchup.
    fn or_insert(self, matchup: Matchup) -> &'a mut Matchup
    {
        match self
        {
            EntryClone::Vacant(mut entry) => {
                entry.insert(matchup);
                entry.reference.table.get_mut(entry.row, entry.column).unwrap()
                },
            EntryClone::Occupied(entry) => {entry.get()}
        }
    }
    */

    /// Returns the pair's matchup, or creates and returns a new one if it didn't exist.
    fn or_create(self) -> &'a mut Matchup
    {
        match self
        {
            EntryClone::Vacant(mut entry) => {
                let (row, column) = (entry.row.clone(), entry.column.clone());
                entry.insert(Matchup::new(row, column));
                entry.reference.table.get_mut(entry.row, entry.column).unwrap()

            },
            EntryClone::Occupied(entry) => {
                entry.get()
            }
        }
    }
}

struct VacantEntryClone<'a, 'b, 'c>
{
    reference: &'a mut SumMatrix,
    row: &'b String,
    column: &'c String
}

impl<'a, 'b, 'c> VacantEntryClone<'a, 'b, 'c>
{
    fn insert(&mut self, val: Matchup)
    {
        self.reference.table.insert(self.row.to_owned(), self.column.to_owned(), val);
    }
}

struct OccupiedEntryClone<'a, 'b, 'c>
{
    reference: &'a mut SumMatrix,
    row: &'b String,
    column: &'c String
}

impl<'a, 'b, 'c> OccupiedEntryClone<'a, 'b, 'c>
{
    fn get(self) -> &'a mut Matchup
    {
        self.reference.table.get_mut(self.row, self.column).unwrap()
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
