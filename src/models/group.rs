use core::fmt;

use serde::{Deserialize, Serialize};

use crate::models::{user::User,expenses::{Expense,Transaction}};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Group{
    pub id: i32,
    pub name: String,
    pub members: Vec<User>,
    pub expenses: Vec<Expense>,
}

#[derive(Debug,Clone)]
pub struct GroupSummary{
    pub group: Group,
    pub total_spent: f64,
    pub expenses: Vec<Expense>,
    pub transactions: Vec<Transaction>,
}
impl fmt::Display for GroupSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group: {}\nTotal Spent: {}\nExpenses: {:?}\nTransactions: {:?}", self.group.name, self.total_spent, self.expenses, self.transactions)
    }
}

impl Group{
    pub fn new(id: i32, name: &str, members: Vec<User>) -> Self {
        Self {
            id,
            name: name.to_string(),
            members,
            expenses: Vec::new(),
        }
    }
    pub fn add_expense(self: &mut Self, expense: Expense){
        self.expenses.push(expense);
    }
    pub fn get_group_summary(self: &Self) -> GroupSummary{
        let mut total_spent = 0.0;
        let mut transactions: Vec<Transaction> = Vec::new();
        for expense in &self.expenses{
            total_spent += expense.amount;
            let share = expense.amount / expense.participants.len() as f64;
            for participant in &expense.participants{
                let transaction = Transaction{
                    id: transactions.len() as i32 + 1,
                    payer: expense.payer.clone(),
                    receiver: participant.clone(),
                    amount: share,
                    date: expense.date.clone(),
                };
                transactions.push(transaction);
            }
        }
        GroupSummary{
            group: self.clone(),
            total_spent,
            expenses: self.expenses.clone(),
            transactions,
        }
    }
}
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group: {}\nMembers: {:?}", self.name, self.members)
    }
}
