namespace ConsoleApp1;

internal static class Program
{
    private const decimal VipDiscountRate = 0.15m;

    public static async Task Main(string[] args)
    {
        string? requestedCustomer = args.FirstOrDefault();
        string customerName = string.IsNullOrWhiteSpace(requestedCustomer)
            ? "CustomerName"
            : requestedCustomer.Trim();

        Order[] orders =
        [
            new(1001, "Mechanical keyboard", 111.1m, OrderStatus.Paid),
            new(1002, "USB-C cable", 22.22m, OrderStatus.Pending),
            new(1003, "Ultrawide monitor", 333.00m, OrderStatus.Paid),
        ];

        var notableOrders = orders
            .Where(static order => order.Total >= 100m)
            .OrderByDescending(static order => order.Total);

        Console.WriteLine($"Hello, {customerName}!");

        foreach (Order order in notableOrders)
        {
            decimal discount = CalculateDiscount(order, isVip: true);
            string category = Describe(order);

            Console.WriteLine(
                $"Order #{order.Id}: {order.Product} costs {order.Total:C2} " +
                $"({category}, discount: {discount:P0})."
            );
        }

        string report = $$"""
            {
              "customer": "{{customerName}}",
              "orderCount": {{orders.Length}},
              "generatedAt": "{{DateTimeOffset.UtcNow:O}}"
            }
            """;

        await Task.Delay(TimeSpan.FromMilliseconds(10));
        Console.WriteLine(report);
    }

    private static decimal CalculateDiscount(Order order, bool isVip) => order switch
    {
        { Status: OrderStatus.Cancelled } => 0m,
        { Total: >= 500m } when isVip => VipDiscountRate,
        { Total: >= 100m } => 0.05m,
        _ => 0m,
    };

    private static string Describe(Order order) => order switch
    {
        { Status: OrderStatus.Paid, Total: >= 500m } => "premium purchase",
        { Status: OrderStatus.Paid } => "completed purchase",
        { Status: OrderStatus.Pending } => "awaiting payment",
        _ => "cancelled order",
    };

    private sealed record Order(int Id, string Product, decimal Total, OrderStatus Status);

    private enum OrderStatus
    {
        Pending,
        Paid,
        Cancelled,
    }
}
