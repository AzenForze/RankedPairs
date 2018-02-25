
mod election;
mod sum_matrix;
mod ranked_pairs;
mod matchup;
mod matchup_result;
mod graph;
mod table;

mod dfs;

use election::Election;
use ranked_pairs::RankedPairs;


fn main()
{
    let mut city_elec = Election::new();

    city_elec.add_ballots("
        42:Memphis>Nashville>Chattanooga>Knoxville
        26:Nashville>Chattanooga>Knoxville>Memphis
        15:Chattanooga>Knoxville>Nashville>Memphis
        17:Knoxville>Chattanooga>Nashville>Memphis");

    let mut basic_5cand = Election::new();

    basic_5cand.add_ballots("
        5:A>C>B>E>D
        5:A>D>E>C>B
        8:B>E>D>A>C
        3:C>A>B>E>D
        7:C>A>E>B>D
        2:C>B>A>D>E
        7:D>C>E>B>A
        8:E>B>A>D>C");

    let mut basic_3cand = Election::new();

    basic_3cand.add_ballots("
        35:B>C>S
        34:C>S>B
        31:S>B>C");

    let ranked_pairs = RankedPairs::with_election(city_elec);

    let winner = match ranked_pairs.get_winner(true)
    {
        Ok(w) => w,
        Err(e) => panic!("{}", e)
    };

    println!("\nWinner: {}", winner);
}
