use sum_matrix::SumMatrix;
use graph::{EmptyGraphError, Graph};
use election::Election;
use matchup_result::MatchupResult;


/**
RankedPairs is a Condorcet-compliant voting method that uses a sum matrix.
Each matchup of a pair of candidates is sorted based on on how strong the winner's victory is (either by the margin or winning votes).
Starting from the strongest victory, draw a path from the loser to the winner, unless it's already possible to get to the loser from the winner (don't allow cycles)
After all matches have been considered, all paths will lead to one candidate, who will have no outgoing paths. They are the winner.
*/
pub struct RankedPairs
{
    sum_matrix: SumMatrix,
    strength_type: StrengthType
}

impl RankedPairs
{
    pub fn with_matrix(sum_matrix: SumMatrix, strength_type: StrengthType) -> Self
    {
        RankedPairs{ sum_matrix: sum_matrix, strength_type: strength_type }
    }

    pub fn with_election(election: Election, strength_type: StrengthType) -> Self
    {
        RankedPairs::with_matrix(election.get_matrix(), strength_type)
    }

    fn ranked_pairs(&self) -> Vec<MatchupResult>
    {
        let mut ranked_pairs: Vec<MatchupResult> = Vec::new();

        for matchup in self.sum_matrix.matchups()
        {
            ranked_pairs.push(matchup.result(self.strength_type.use_margin()));
        }

        ranked_pairs.sort();
        ranked_pairs.reverse();

        return ranked_pairs;
    }
    
    /**
    Gets the RankedPairs winner of the SumMatrix.
    */
    pub fn get_winner(&self) -> Result<String, EmptyGraphError>
    {
        let ranked_pairs = self.ranked_pairs();

        /*
        println!("Ranked Pairs: ");

        for m in ranked_pairs.iter()
        {
            println!("{}", m);
        }
        */

        let mut graph = Graph::new();

        for result in ranked_pairs
        {
            result.try_lock_in(&mut graph);
        }
        
        //println!("Graph:\n{}", graph);

        graph.find_sink()
    }
}

pub enum StrengthType
{
    Margin,
    WinningVotes
}

impl StrengthType
{
    pub fn use_margin(&self) -> bool
    {
        match *self
        {
            StrengthType::Margin => true,
            StrengthType::WinningVotes => false
        }
    }
}
