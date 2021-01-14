use std::collections::HashMap;

// this method will calculate cash flow for the user 
// (banks that are customers of Plaid for example)
// add all income per month
// add all debts per month
// subtract total income from total debt to find 
// cash flow per month
pub fn calc_cashflow(income: HashMap<String, f32>, debts: HashMap<String, f32>) -> f32{
    
    let mut income_total: f32 = 0.0;
    let mut debt_total: f32 = 0.0;

    for (_income_name, income_amount) in &income
    {
        income_total += income_amount;
    }

    for (_debt_name, debt_amount)in &debts
    {
        debt_total += debt_amount;
    }
    
    let cash_flow: f32 = income_total - debt_total;
    
    return cash_flow;
}
// this method will calculate debt to income ratio for the user 
// (banks that are customers of Plaid for example)
// add all income per month
// add all debts per month
// divide debt by income to find 
// DTR
pub fn calc_debt_income_ratio (income: HashMap<String, f32>, debts: HashMap<String, f32>) -> f32{

    let mut income_total: f32 = 0.0;
    let mut debt_total: f32 = 0.0;

    for (_income_name, income_amount) in &income
    {
        income_total += income_amount;
    }

    for (_debt_name, debt_amount)in &debts
    {
        debt_total += debt_amount;
    }
    
    let dtr: f32 = debt_total/income_total;
    
    return dtr;
}

pub fn main(){

    let mut income: HashMap<String, f32> = HashMap::new();
    let mut debts: HashMap<String, f32> = HashMap::new();

    income.insert(String::from("rental 1"), 2500.00);
    income.insert(String::from("rental 2"), 2700.00);
    income.insert(String::from("rental 3"), 3000.00);
    income.insert(String::from("rental 4"), 3500.00);

    debts.insert(String::from("rental tax 1"), 500.00);
    debts.insert(String::from("rental mortgage 1"), 1300.00);
    debts.insert(String::from("rental tax 2"), 600.00);
    debts.insert(String::from("rental mortgage 2"), 1400.00);
    debts.insert(String::from("rental tax 3"), 700.00);
    debts.insert(String::from("rental mortgage 3"), 1500.00);
    debts.insert(String::from("rental tax 4"), 800.00);
    debts.insert(String::from("rental mortgage 4"), 1600.00);
    
    // cloing HashMaps here so it does not go out of scope when we call cash_flow..
    let dtr: f32 = calc_debt_income_ratio(income.clone(), debts.clone());
    println!("debt to income ratio: {}%", dtr * 100.0);

    let cash_flow: f32 = calc_cashflow(income, debts);
    println!("cash flow: ${}", cash_flow);
}

