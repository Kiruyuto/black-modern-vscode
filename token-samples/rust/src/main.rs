use std::{
    env, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const VIP_DISCOUNT_RATE: f64 = 0.15;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OrderStatus {
    Pending,
    Paid,
    Cancelled,
}

#[derive(Debug)]
struct Order {
    id: u32,
    product: &'static str,
    total: f64,
    status: OrderStatus,
}

fn main() {
    let requested_customer = env::args().nth(1);
    let customer_name = requested_customer
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("CustomerName");

    let orders = [
        Order {
            id: 1001,
            product: "Mechanical keyboard",
            total: 111.1,
            status: OrderStatus::Paid,
        },
        Order {
            id: 1002,
            product: "USB-C cable",
            total: 22.22,
            status: OrderStatus::Pending,
        },
        Order {
            id: 1003,
            product: "Ultrawide monitor",
            total: 333.0,
            status: OrderStatus::Paid,
        },
    ];

    let mut notable_orders: Vec<&Order> = orders
        .iter()
        .filter(|order| order.total >= 100.0)
        .collect();

    notable_orders.sort_by(|left, right| right.total.total_cmp(&left.total));

    println!("Hello, {customer_name}!");

    for order in notable_orders {
        let discount = calculate_discount(order, true);
        let category = describe(order);

        println!(
            "Order #{}: {} costs ${:.2} ({category}, discount: {:.0}%).",
            order.id,
            order.product,
            order.total,
            discount * 100.0,
        );
    }

    let generated_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let report = format!(
        r#"{{
  "customer": "{customer_name}",
  "orderCount": {},
  "generatedAt": {generated_at}
}}"#,
        orders.len(),
    );

    thread::sleep(Duration::from_millis(10));
    println!("{report}");
}

fn calculate_discount(order: &Order, is_vip: bool) -> f64 {
    match order {
        Order {
            status: OrderStatus::Cancelled,
            ..
        } => 0.0,
        Order { total, .. } if *total >= 500.0 && is_vip => VIP_DISCOUNT_RATE,
        Order { total, .. } if *total >= 100.0 => 0.05,
        _ => 0.0,
    }
}

fn describe(order: &Order) -> &'static str {
    match order {
        Order {
            status: OrderStatus::Paid,
            total,
            ..
        } if *total >= 500.0 => "premium purchase",
        Order {
            status: OrderStatus::Paid,
            ..
        } => "completed purchase",
        Order {
            status: OrderStatus::Pending,
            ..
        } => "awaiting payment",
        _ => "cancelled order",
    }
}
