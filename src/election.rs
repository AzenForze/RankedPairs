
use sum_matrix::SumMatrix;

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

    pub fn get_matrix(&self) -> SumMatrix
    {
        SumMatrix::new(&self.ballots)
    }

    /**
    Parses a String vote.
    "A>B=C>D" creates [[a], [b, c], [d]]
    */
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

    /**
    Parses a String set of ballots.
    Format:
    [count]:[vote]
    (See self.add_vote() for [vote])
    */
    pub fn add_ballots(&mut self, data: &str)
    {
        for line in data.trim().lines()
        {
            let mut parts = line.trim().split(':');

            let count_str = match parts.next()
            {
                Some(count_str) => count_str,
                None => panic!("Wrong ballot format")
            };
            
            let vote = match parts.next()
            {
                Some(vote) => vote,
                None => panic!("Wrong ballot format")
            };

            match parts.next()
            {
                Some(_) => panic!("Wrong ballot format"),
                None => {}
            }
            
            let count = match count_str.parse::<u32>()
            {
                Ok(count) => count,
                Err(e) => panic!("{}; string was: {}", e, count_str)
            };

            for _ in 0..count
            {
                self.add_vote(vote);
            }
        }
    }
}
