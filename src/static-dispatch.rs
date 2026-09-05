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

// ==================================== //
// 3. Define the Context using Generics //
// ==================================== //

struct CheckoutContext<T: PaymentStrategy> {
    payment_strategy: T, // similar to bridge pattern
}

impl<T: PaymentStrategy> CheckoutContext<T> {
    fn new(payment_strategy: T) -> Self {
        Self { payment_strategy }
    }

    fn complete_purchase(&self, amount: u32) -> String {
        self.payment_strategy.pay(amount)
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    {
        // Credit Card
        let card_checkout = CheckoutContext::new(CreditCard {
            card_number: String::from("1234-5678"),
        });
        println!("{}", card_checkout.complete_purchase(100));
    }

    {
        // PayPal
        let paypal_checkout = CheckoutContext::new(PayPal {
            email: String::from("user@example.com"),
        });
        println!("{}", paypal_checkout.complete_purchase(250));
    }
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
    fn checkout_context_dispatches_to_injected_strategy() {
        let card = CheckoutContext::new(CreditCard {
            card_number: String::from("1234-5678"),
        });
        assert_eq!(
            card.complete_purchase(100),
            "Paid 100 using Credit Card (1234-5678)"
        );

        let paypal = CheckoutContext::new(PayPal {
            email: String::from("user@example.com"),
        });
        assert_eq!(
            paypal.complete_purchase(250),
            "Paid 250 using PayPal (user@example.com)"
        );
    }
}
