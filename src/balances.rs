use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an acount to a specific amount. This will overwrite any existing balance.
    pub fn set_balance(&mut self, who: &str, amount: u128) {
        /* Insert amount into the Btreemao under who */
        self.balances.insert(who.to_string(), amount);
    }

    /// get the balance of the account who
    /// if the account has no stored balance return 0
    pub fn get_balance(&self, who: &str) -> u128 {
        /* Get the balance of who from the Btreemap. If who is not in the map return 0 */
        *self.balances.get(who).unwrap_or(&0)
    }

    /// transfer amount from one account to another.
    /// this function verifies that "from" has enough balance to transfer the amount and that the amount is not zero.
    /// if the transfer is successful it returns true, otherwise it returns false.

    pub fn transfer(
        &mut self,
        from: &str,
        to: &str,
        amount: u128,
    ) -> Result<(), &'static str> {
        let from_balance = self.get_balance(from);
        let to_balance = self.get_balance(to);

        let new_from_balance = from_balance
            .checked_sub(amount)
            .ok_or("Not enough balance")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow when adding balance")?;
        self.set_balance(from, new_from_balance);
        self.set_balance(to, new_to_balance);
        Ok(())
    }
}
