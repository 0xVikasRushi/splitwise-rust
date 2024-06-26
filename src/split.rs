use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    pub name: String,
}

impl User {
    pub fn create_user(name: &str) -> User {
        User {
            name: String::from(name),
        }
    }
}
pub struct Transaction {
    pub from: User,
    pub to: User,
    pub amount: f64,
}

impl Transaction {
    pub fn new(from: User, to: User, amount: f64) -> Transaction {
        Transaction { from, to, amount }
    }
}

pub struct Transactions {
    pub transactions: Vec<Transaction>,
}

#[derive(Clone)]
pub struct Net {
    pub user: User,
    pub amount: f32,
}

impl Transactions {
    pub fn new() -> Transactions {
        Transactions {
            transactions: Vec::new(),
        }
    }

    pub fn add(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }

    pub fn display(&self) {
        println!("{0: <10} | {1: <10} | {2: <10}", "Who", "Owes To", "Amount");
        for i in &self.transactions {
            println!(
                "{0: <10} | {1: <10} | {2: <10}",
                i.to.name, i.from.name, i.amount
            );
        }
    }

    fn calc_net(&self) -> (Vec<Net>, Vec<Net>) {
        let mut map: HashMap<String, f32> = HashMap::new();

        for i in &self.transactions {
            let from = i.from.name.clone();
            let to = i.to.name.clone();
            let amount = i.amount as f32;

            *map.entry(from.clone()).or_insert(0.0) -= amount;
            *map.entry(to.clone()).or_insert(0.0) += amount;
        }

        let mut net_positive: Vec<Net> = Vec::new();
        let mut net_negative: Vec<Net> = Vec::new();

        for (name, net_amount) in map {
            if net_amount > 0.0 {
                net_positive.push(Net {
                    user: User::create_user(&name),
                    amount: net_amount,
                });
            } else if net_amount < 0.0 {
                net_negative.push(Net {
                    user: User::create_user(&name),
                    amount: net_amount,
                });
            }
        }
        (net_positive, net_negative)
    }

    fn split(&self, positive: &mut Vec<Net>, negative: &mut Vec<Net>) -> Transactions {
        let mut answer = Transactions::new();

        while !positive.is_empty() && !negative.is_empty() {
            let mut pos = positive.pop().unwrap();
            let mut neg = negative.pop().unwrap();

            let settle_amount = if -neg.amount > pos.amount {
                pos.amount
            } else {
                -neg.amount
            };

            pos.amount -= settle_amount;
            neg.amount += settle_amount;

            answer.add(Transaction::new(
                neg.user.clone(),
                pos.user.clone(),
                settle_amount as f64,
            ));

            if pos.amount > 0.0 {
                positive.push(pos);
            }
            if neg.amount < 0.0 {
                negative.push(neg);
            }
        }

        answer
    }

    pub fn split_bill(&self) -> Transactions {
        let (mut pos, mut neg) = self.calc_net();
        self.split(&mut pos, &mut neg)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic_flow() {
        let user1 = User::create_user("Alice");
        let user2 = User::create_user("Bob");
        let user3 = User::create_user("Charlie");

        let mut all_transactions = Transactions::new();

        let tx1 = Transaction::new(user1.clone(), user2.clone(), 10.0);
        let tx2 = Transaction::new(user2.clone(), user1.clone(), 1.0);
        let tx3 = Transaction::new(user2.clone(), user3.clone(), 5.0);
        let tx4 = Transaction::new(user3.clone(), user1.clone(), 5.0);

        all_transactions.add(tx1);
        all_transactions.add(tx2);
        all_transactions.add(tx3);
        all_transactions.add(tx4);
        println!();

        let settled_transactions = all_transactions.split_bill();
        let expected_tx = Transaction::new(user1, user2, 4.0);

        assert_eq!(settled_transactions.transactions[0].from, expected_tx.from);
        assert_eq!(settled_transactions.transactions[0].to, expected_tx.to);
        assert_eq!(
            settled_transactions.transactions[0].amount,
            expected_tx.amount
        );

        println!("< --------------------SPLIT BILL OUTPUT -------------------------->");
        println!();
        settled_transactions.display();
    }
}
