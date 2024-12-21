/// This module defines two enums, `Tier` and `Subscription`, which represent different levels of access to a site's premium features.
///
/// The `Tier` enum represents the different tiers of premium access, with `Gold`, `Silver`, and `Platinum` as the available options.
///
/// The `Subscription` enum represents the different types of subscriptions a user can have, with `Free`, `Basic`, and `Premium` as the available options. The `Basic` variant includes a price and number of months, while the `Premium` variant includes a `Tier`.
///
/// The `Subscription` enum also includes a `summarize()` method, which prints a human-readable summary of the current subscription type, including the tier for premium subscriptions.

#[derive(Debug)] // Derive the Debug trait for the Tier enum
#[allow(dead_code)] // Allow dead code for the Tier enum
enum Tier {
    Gold,     // Gold tier
    Silver,   // Silver tier
    Platinum, // Platinum tier
}

#[derive(Debug)] // Derive the Debug trait for the Subscription enum
#[allow(dead_code)] // Allow dead code for the Subscription enum
enum Subscription {
    Free,                   // Free subscription
    Basic(f64, u32),        // Basic subscription (price, number of months)
    Premium { tier: Tier }, // Premium subscription (tier)
}

impl Subscription {
    /// Prints a summary of the current subscription type.
    ///
    /// This method is an implementation detail of the `Subscription` enum, and is not part of the public API.
    /// It is used to print a human-readable summary of the current subscription type, including the tier for premium subscriptions.
    fn summarize(&self) {
        match self {
            Subscription::Free => println!("You have limited access to the site"), // Print summary for Free subscription
            Subscription::Basic(price, number_of_months) => println!(
                "You have limited access to the site's premium features for {price} USD per month for a total of {number_of_months} months" // Print summary for Basic subscription
            ),
            Subscription::Premium { tier } => {
                println!("You have access to the site's premium features, Your tier is {tier:?}")
            } // Print summary for Premium subscription
        }
    }
}
fn main() {
    let free_subscription = Subscription::Free; // Create a Free subscription
    let basic_subscription = Subscription::Basic(10.0, 12); // Create a Basic subscription
    let premium_subscription = Subscription::Premium { tier: Tier::Gold }; // Create a Premium subscription
    free_subscription.summarize(); // Print summary of Free subscription
    basic_subscription.summarize(); // Print summary of Basic subscription
    premium_subscription.summarize(); // Print summary of Premium subscription
}
