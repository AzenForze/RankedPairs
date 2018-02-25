
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
            let parts: Vec<_> = line.trim().split(':').collect();

            assert_eq!(parts.len(), 2);

            let mut iter = parts.into_iter();
            
            let part = iter.next().unwrap();

            let count = match part.parse::<u32>()
            {
                Ok(i) => i,
                Err(e) => panic!("{}; string was: {}", e, part)
            };
            
            let part = iter.next().unwrap();

            for _ in 0..count
            {
                self.add_vote(part);
            }
        }
    }
}
