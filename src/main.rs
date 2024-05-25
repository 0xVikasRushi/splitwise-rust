use std::collections::HashMap;

#[derive(Clone)]
struct User {
    name: String,
}

impl User {
    fn create_user(name: &str) -> User {
        User {
            name: String::from(name),
        }
    }
}

struct Transaction {
    from: User,
    to: User,
    amount: u64,
}

impl Transaction {
    fn new(from: User, to: User, amount: u64) -> Transaction {
        Transaction { from, to, amount }
    }
}

struct Transactions {
    transactions: Vec<Transaction>,
}

struct Net {
    user: User,
    amount: i32,
}

impl Transactions {
    fn new() -> Transactions {
        Transactions {
            transactions: Vec::new(),
        }
    }
    fn add(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }

    fn display(&mut self) {
        for i in &self.transactions {
            println!(
                "------> from : {} ,to : {}, amount ,{}",
                i.from.name, i.to.name, i.amount
            );
        }
    }

    fn calc_net(&mut self) -> (Vec<Net>, Vec<Net>) {
        let mut map: HashMap<String, i32> = HashMap::new();

        for i in &self.transactions {
            let from = i.from.name.clone();
            let to = i.to.name.clone();
            let amount = i.amount.clone() as i32;
            *map.entry(from.clone()).or_insert(0) -= amount;
            *map.entry(to.clone()).or_insert(0) += amount;
        }

        let mut net_positive: Vec<Net> = Vec::new();
        let mut net_negative: Vec<Net> = Vec::new();

        for (name, net_amount) in map {
            if net_amount > 0 {
                net_positive.push(Net {
                    user: User::create_user(&name),
                    amount: net_amount,
                });
            } else if net_amount < 0 {
                net_negative.push(Net {
                    user: User::create_user(&name),
                    amount: net_amount,
                });
            }
        }
        return (net_positive, net_negative);
    }

    fn split(positive: Vec<Net>, negative: Vec<Net>) -> Transactions {
        let answer = Transactions::new();
        answer
    }
}

fn main() {
    let user1 = User::create_user("Alice");
    let user2 = User::create_user("Bob");
    let user3 = User::create_user("Charlie");
    let user4 = User::create_user("Dave");

    let mut all_transactions = Transactions::new();

    let tx1 = Transaction::new(user1.clone(), user2.clone(), 10);
    let tx2 = Transaction::new(user2.clone(), user3.clone(), 20);
    let tx3 = Transaction::new(user3.clone(), user4.clone(), 30);
    let tx4 = Transaction::new(user1.clone(), user2.clone(), 40);

    all_transactions.add(tx1);
    all_transactions.add(tx2);
    all_transactions.add(tx3);
    all_transactions.add(tx4);

    all_transactions.display();
    let (a, b) = all_transactions.calc_net();
}
