mod test;

fn main() {}

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let result = accounts
        .into_iter()
        .map(|vector| {
            vector
                .into_iter()
                .reduce(|a, b| a + b)
                .expect("the iterator is empty")
        })
        .collect::<Vec<i32>>();

    result.into_iter().max().expect("the iterator is empty")
}
