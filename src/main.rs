
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
        17:Knoxville>Chattanooga>Nashville>Memphis".to_string());

    let ranked_pairs = RankedPairs::with_election(city_elec);

    let winner = match ranked_pairs.get_winner(true)
    {
        Ok(w) => w,
        Err(e) => panic!("{}", e)
    };

    println!("\nWinner: {}", winner);
    
    /*
    let mut e = Election::new();

    e.add_ballots("
    35:B>C>S
    34:C>S>B
    31:S>B>C".to_owned());

    let rp = RankedPairs::with_election(e);
    let winner = match rp.get_winner(false)
    {
        Ok(w) => w,
        Err(e) => panic!("{}", e)
    };

    println!("\nWinner: {}", winner);

    let mut e = Election::new();

    e.add_ballots("
        5:A>C>B>E>D
        5:A>D>E>C>B
        8:B>E>D>A>C
        3:C>A>B>E>D
        7:C>A>E>B>D
        2:C>B>A>D>E
        7:D>C>E>B>A
        8:E>B>A>D>C".to_string());

    let mut city_elec = Election::new();

    city_elec.add_ballots("
        42:Memphis>Nashville>Chattanooga>Knoxville
        26:Nashville>Chattanooga>Knoxville>Memphis
        15:Chattanooga>Knoxville>Nashville>Memphis
        17:Knoxville>Chattanooga>Nashville>Memphis".to_string());
    
    
    let ntrp = RankedPairs::new();
    let matrix = e.get_matrix();
    
    let winner = match ntrp.get_winner(&matrix, false)
    {
        Ok(w) => w,
        Err(e) => panic!("{}", e)
    };
    
    
    println!("\n\nWinner: {}", winner);
    
    let winner = match ntrp.get_winner(&city_elec.get_matrix(), false)
    {
        Ok(w) => w,
        Err(e) => panic!("{}", e)
    };

    println!("\n\nWinner: {}", winner);
    */
}
