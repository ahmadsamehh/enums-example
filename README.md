# Enums Example

This Rust project demonstrates the use of enums to represent different subscription levels and tiers for a hypothetical website.

## Overview

The code defines two enums:

* **`Tier`**: Represents the different tiers of premium access available (Gold, Silver, Platinum).
* **`Subscription`**: Represents the different types of subscriptions a user can have (Free, Basic, Premium).  The `Basic` variant includes price and duration information, while the `Premium` variant includes the user's `Tier`.

The `Subscription` enum also includes a `summarize()` method that prints a user-friendly description of the subscription.

## Usage

The `main` function creates instances of each subscription type and calls the `summarize()` method to demonstrate their functionality.

```bash
cargo run
