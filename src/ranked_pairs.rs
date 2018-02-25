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
    sum_matrix: SumMatrix
}

impl RankedPairs
{
    pub fn with_matrix(sum_matrix: SumMatrix) -> Self
    {
        RankedPairs{ sum_matrix: sum_matrix }
    }
    
    pub fn with_election(election: Election) -> Self
    {
        RankedPairs::with_matrix(election.get_matrix())
    }

    fn ranked_pairs(&self, use_margins: bool) -> Vec<MatchupResult>
    {
        let mut ranked_pairs: Vec<MatchupResult> = Vec::new();

        for matchup in self.sum_matrix.matchups()
        {
            ranked_pairs.push(matchup.result());
        }

        ranked_pairs.sort_by( |a, b| b.cmp(&a, use_margins));

        return ranked_pairs;
    }

    /**
    Gets the RankedPairs winner of the SumMatrix.
    */
    pub fn get_winner(&self, use_margins: bool) -> Result<String, EmptyGraphError>
    {
        let ranked_pairs = self.ranked_pairs(use_margins);
        
        // Debug output
        println!("Ranked Pairs: ");

        for m in ranked_pairs.iter()
        {
            println!("{}", m);
        }
        
        let mut graph = Graph::new();

        for m in ranked_pairs
        {
            m.try_lock_in(&mut graph);
        }
        
        // Debug output
        println!("Graph:\n{}", graph);

        graph.find_sink()   
    }
}