#[cfg(test)]
use std::collections::HashMap;
use my_rust_project::cart_management::cart_management::{Cart, CartItem};
use rust_decimal_macros::dec;

#[test]
fn test_add_item_to_cart() {

  let mut cart = Cart::new();

  // Add an item to the cart
  cart.add_item("product_123".to_string(), 2);

  // Verify the item was added with the correct quantity
  assert_eq!(cart.items.len(), 1);
  assert_eq!(cart.items.get("product_123"), Some(&2));
}

#[test]
fn test_remove_item() {
    let mut cart = Cart::new();
    cart.add_item("product_123".to_string(), 2);
    cart.add_item("product_456".to_string(), 3);

    // Ensure items are added
    assert_eq!(cart.items.len(), 2);

    // Remove one product
    cart.remove_item("product_123".to_string());

    // Verify the product is removed
    assert_eq!(cart.items.len(), 1);
    assert!(!cart.items.contains_key("product_123"));
    assert!(cart.items.contains_key("product_456"));
}

#[test]
fn test_get_items() {
    let mut cart = Cart::new();
    cart.add_item("product_123".to_string(), 2);
    cart.add_item("product_456".to_string(), 3);

    let items = cart.get_items();

    // Verify the number of items in the cart
    assert_eq!(items.len(), 2);

    // Verify the details of each item
    assert!(items.contains(&CartItem {
        product_id: "product_123".to_string(),
        quantity: 2,
    }));
    assert!(items.contains(&CartItem {
        product_id: "product_456".to_string(),
        quantity: 3,
    }));
}

#[test]
fn test_clear_cart() {
    let mut cart = Cart::new();
    cart.add_item("product_123".to_string(), 2);
    cart.add_item("product_456".to_string(), 3);

    // Ensure items are added
    assert_eq!(cart.items.len(), 2);

    // Clear the cart
    cart.clear_cart();

    // Verify the cart is empty
    assert_eq!(cart.items.len(), 0);
}

#[test]
fn test_get_total_price() {
    let mut cart = Cart {
        items: HashMap::new(),
        product_prices: HashMap::new(),
    };

    // Add product prices
    cart.product_prices.insert("product_123".to_string(), dec!(10.50));
    cart.product_prices.insert("product_456".to_string(), dec!(20.00));

    // Add items to the cart
    cart.add_item("product_123".to_string(), 2); // 2 * 10.50 = 21.00
    cart.add_item("product_456".to_string(), 3); // 3 * 20.00 = 60.00

    // Calculate total price
    let total_price = cart.get_total_price();

    // Verify the total price
    assert_eq!(total_price, dec!(81.00)); // 21.00 + 60.00 = 81.00
}