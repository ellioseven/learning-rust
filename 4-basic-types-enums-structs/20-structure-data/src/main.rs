struct Customer {
    name: String
}

struct Order {
    id: i32,
    customer: Customer
}

fn main () {
    let order = Order {
        id: 10,
        customer: Customer {
            name: String::from("Elliot Mitchum")
        }
    };

    println!("Order - id: {}, name: {}", order.id, order.customer.name);
}
