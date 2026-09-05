// ================================ //
// 1. Define the Strategy Interface //
// ================================ //

trait PaymentStrategy {
    fn pay(&self, amount: u32) -> String;
}

// ================================ //
// 2. Implement Concrete Strategies //
// ================================ //

// ============= //
// A. CreditCard //
// ============= //

struct CreditCard {
    card_number: String,
}

impl PaymentStrategy for CreditCard {
    fn pay(&self, amount: u32) -> String {
        format!("Paid {} using Credit Card ({})", amount, self.card_number)
    }
}

// ========= //
// B. PayPal //
// ========= //

struct PayPal {
    email: String,
}

impl PaymentStrategy for PayPal {
    fn pay(&self, amount: u32) -> String {
        format!("Paid {} using PayPal ({})", amount, self.email)
    }
}

// ========================================= //
// 3. Define the Context using Trait Objects //
// ========================================= //

struct DynamicCheckout {
    strategy: Box<dyn PaymentStrategy>,
}

impl DynamicCheckout {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        Self { strategy }
    }

    fn complete_purchase(&self, amount: u32) -> String {
        self.strategy.pay(amount)
    }

    // swaps the strategy at runtime
    // TODO: could also return the old payment strategy using e.g. `.take()`
    fn change_strategy(&mut self, new_strategy: Box<dyn PaymentStrategy>) {
        self.strategy = new_strategy;
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    // Initialize with Credit Card
    let mut checkout = DynamicCheckout::new(Box::new(CreditCard {
        card_number: String::from("4321-8765"),
    }));
    println!("{}", checkout.complete_purchase(50));

    // Swap to PayPal at runtime
    checkout.change_strategy(Box::new(PayPal {
        email: String::from("switch@example.com"),
    }));

    println!("{}", checkout.complete_purchase(75));
}

// ===== //
// Tests //
// ===== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credit_card_returns_formatted_receipt() {
        let strategy = CreditCard {
            card_number: String::from("1234-5678"),
        };
        assert_eq!(strategy.pay(100), "Paid 100 using Credit Card (1234-5678)");
    }

    #[test]
    fn paypal_returns_formatted_receipt() {
        let strategy = PayPal {
            email: String::from("user@example.com"),
        };
        assert_eq!(strategy.pay(250), "Paid 250 using PayPal (user@example.com)");
    }

    #[test]
    fn dynamic_checkout_can_swap_strategy_at_runtime() {
        let mut checkout = DynamicCheckout::new(Box::new(CreditCard {
            card_number: String::from("4321-8765"),
        }));
        assert_eq!(
            checkout.complete_purchase(50),
            "Paid 50 using Credit Card (4321-8765)"
        );

        checkout.change_strategy(Box::new(PayPal {
            email: String::from("switch@example.com"),
        }));
        assert_eq!(
            checkout.complete_purchase(75),
            "Paid 75 using PayPal (switch@example.com)"
        );
    }
}
