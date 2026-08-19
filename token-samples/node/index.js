"use strict";

const vipDiscountRate = 0.15;

const OrderStatus = Object.freeze({
    pending: "pending",
    paid: "paid",
    cancelled: "cancelled",
});

const currencyFormatter = new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
});

class Order {
    constructor(id, product, total, status) {
        this.id = id;
        this.product = product;
        this.total = total;
        this.status = status;

        Object.freeze(this);
    }
}

async function main(args) {
    const requestedCustomer = args.at(0);
    const customerName = !requestedCustomer?.trim()
        ? "CustomerName"
        : requestedCustomer.trim();

    const orders = [
        new Order(1001, "Mechanical keyboard", 111.1, OrderStatus.paid),
        new Order(1002, "USB-C cable", 22.22, OrderStatus.pending),
        new Order(1003, "Ultrawide monitor", 333.0, OrderStatus.paid),
    ];

    const notableOrders = orders
        .filter((order) => order.total >= 100)
        .sort((left, right) => right.total - left.total);

    console.log(`Hello, ${customerName}!`);

    for (const order of notableOrders) {
        const discount = calculateDiscount(order, true);
        const category = describe(order);

        console.log(
            `Order #${order.id}: ${order.product} costs ` +
            `${currencyFormatter.format(order.total)} ` +
            `(${category}, discount: ${Math.round(discount * 100)}%).`,
        );
    }

    const report = `{
  "customer": "${customerName}",
  "orderCount": ${orders.length},
  "generatedAt": "${new Date().toISOString()}"
}`;

    await delay(10);
    console.log(report);
}

function calculateDiscount(order, isVip) {
    if (order.status === OrderStatus.cancelled) {
        return 0;
    }

    if (order.total >= 500 && isVip) {
        return vipDiscountRate;
    }

    return order.total >= 100 ? 0.05 : 0;
}

function describe(order) {
    if (order.status === OrderStatus.paid && order.total >= 500) {
        return "premium purchase";
    }

    switch (order.status) {
        case OrderStatus.paid:
            return "completed purchase";
        case OrderStatus.pending:
            return "awaiting payment";
        default:
            return "cancelled order";
    }
}

function delay(milliseconds) {
    return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

main(process.argv.slice(2)).catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
