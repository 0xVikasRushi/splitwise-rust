#[derive(Clone)]
struct User {
    id: u8,
    name: String,
}

impl User {
    fn create_user(id: u8, name: &str) -> User {
        User {
            id,
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

impl Transactions {
    fn new() -> Transactions {
        Transactions {
            transactions: Vec::new(),
        }
    }
    fn add(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }
}

fn main() {
    let user1 = User::create_user(1, "Alice");
    let user2 = User::create_user(2, "Bob");
    let user3 = User::create_user(3, "Charlie");
    let user4 = User::create_user(4, "Dave");

    let mut all_transactions = Transactions::new();

    let tx1 = Transaction::new(user1.clone(), user2.clone(), 10);
    let tx2 = Transaction::new(user2.clone(), user3.clone(), 20);
    let tx3 = Transaction::new(user3.clone(), user4.clone(), 30);
    let tx4 = Transaction::new(user1.clone(), user2.clone(), 40);

    all_transactions.add(tx1);
    all_transactions.add(tx2);
    all_transactions.add(tx3);
    all_transactions.add(tx4);
}
