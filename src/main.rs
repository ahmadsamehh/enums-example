/// This module defines two enums, `Tier` and `Subscription`, which represent different levels of access to a site's premium features.
///
/// The `Tier` enum represents the different tiers of premium access, with `Gold`, `Silver`, and `Platinum` as the available options.
///
/// The `Subscription` enum represents the different types of subscriptions a user can have, with `Free`, `Basic`, and `Premium` as the available options. The `Basic` variant includes a price and number of months, while the `Premium` variant includes a `Tier`.
///
/// The `Subscription` enum also includes a `summarize()` method, which prints a human-readable summary of the current subscription type, including the tier for premium subscriptions.
#[derive(Debug)]
#[allow(dead_code)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}
#[derive(Debug)]
#[allow(dead_code)]
enum Subscription {
    Free,
    Basic(f64, u32), //price,number of months
    Premium { tier: Tier },
}

impl Subscription {
    /// Prints a summary of the current subscription type.
    ///
    /// This method is an implementation detail of the `Subscription` enum, and is not part of the public API.
    /// It is used to print a human-readable summary of the current subscription type, including the tier for premium subscriptions.
    fn summarize(&self) {
        match self {
            Subscription::Free => println!("You have limited access to the site"),
            Subscription::Basic(price, number_of_months) => println!(
                "You have limited access to the site's premium features for {price} USD per month for a total of {number_of_months} months"
            ),
            Subscription::Premium { tier } => println!("You have access to the site's premium features, Your tier is {tier:?}"),
        }
    }
}
fn main() {
    let free_subscription = Subscription::Free;
    let basic_subscription = Subscription::Basic(10.0, 12);
    let premium_subscription = Subscription::Premium { tier: Tier::Gold };
    free_subscription.summarize();
    basic_subscription.summarize();
    premium_subscription.summarize();
}
