#[cfg(test)]
use my_rust_project::product_manager::product_manager::ProductManager;
use rust_decimal::Decimal;

#[test]
fn test_add_product() {
    let mut product_manager = ProductManager::new();
    product_manager.add_product("product1".to_string(), Decimal::new(1999, 2));
    assert_eq!(
        product_manager.get_product_price("product1".to_string()),
        Some(Decimal::new(1999, 2))
    );
}

#[test]
fn test_get_product_price_nonexistent() {
    let product_manager = ProductManager::new();
    assert_eq!(
        product_manager.get_product_price("nonexistent".to_string()),
        None
    );
}