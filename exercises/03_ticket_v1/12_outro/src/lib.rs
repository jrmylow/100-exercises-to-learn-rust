// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Order {
        Self::validate_name(&product_name);
        Self::validate_qty(&quantity);
        Self::validate_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price
        }
    }
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn set_product_name(&mut self, new_name: String) {
        Self::validate_name(&new_name);
        self.product_name = new_name;
    }
    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }
    pub fn set_quantity(&mut self, new_qty: i32) {
        Self::validate_qty(&new_qty);
        self.quantity = new_qty;
    }
    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }
    pub fn set_unit_price(&mut self, new_price: i32) {
        Self::validate_price(&new_price);
        self.unit_price = new_price;
    }
    pub fn total(&self) -> i32 {
        self.quantity() * self.unit_price()
    }


    fn validate_name(name: &String) {
        if name.is_empty() {
            panic!("Name cannot be empty");
        }
        if name.len() > 300 {
            panic!("Name cannot be longer than 300 characters");
        }
    }
    fn validate_qty(quantity: &i32) {
        if *quantity <= 0 {
            panic!("Quantity must be strictly greater than zero")
        }
    }
    fn validate_price(price: &i32) {
        if *price <= 0 {
            panic!("Price must be strictly greater than zero")
        }
    }
}