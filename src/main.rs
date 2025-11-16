use std::time::{Duration, Instant};

/// -------------------------------
/// Reqon Dynamic Pricing Vending Machine
/// Author: Newton's Bro 😎🔥
/// -------------------------------
///
/// Features:
/// - Stock-based price increase
/// - Demand-rate based price increase (FOREX-like)
/// - Bonding-curve formula for exponential pricing
/// - Purchase tracking
/// - Clean commented Rust code
///
/// This file is all you need to understand how to build
/// a smart vending machine pricing system.
/// -------------------------------

#[derive(Debug)]
struct VendingMachine {
    base_price: f64,
    total_stock: u32,
    remaining_stock: u32,
    purchases: u32,
    last_purchase_time: Instant,
    demand_window: Vec<Instant>, // Tracks purchase timestamps
}

impl VendingMachine {
    fn new(base_price: f64, total_stock: u32) -> Self {
        Self {
            base_price,
            total_stock,
            remaining_stock: total_stock,
            purchases: 0,
            last_purchase_time: Instant::now(),
            demand_window: Vec::new(),
        }
    }

    // ------------------------------------------
    // 1. STOCK-BASED PRICING
    // ------------------------------------------
    fn price_stock_based(&self) -> f64 {
        if self.remaining_stock == 0 {
            return self.base_price * 3.0; // Emergency surge price
        }

        let stock_ratio = self.remaining_stock as f64 / self.total_stock as f64;
        self.base_price * (1.0 + (1.0 - stock_ratio))
    }

    // ------------------------------------------
    // 2. DEMAND-BASED PRICING (FOREX STYLE)
    // ------------------------------------------
    fn price_demand_based(&self) -> f64 {
        let now = Instant::now();

        // Count purchases in the last 30 seconds
        let demand_count: u32 = self
            .demand_window
            .iter()
            .filter(|&&t| now.duration_since(t).as_secs() <= 30)
            .count() as u32;

        let k = 0.05; // sensitivity

        self.base_price * (1.0 + k * demand_count as f64)
    }

    // ------------------------------------------
    // 3. BONDING CURVE PRICING (ADVANCED)
    // price = a * (supply_sold ^ b)
    // ------------------------------------------
    fn price_bonding_curve(&self) -> f64 {
        let a = 2.0;
        let b = 1.3;

        let sold = (self.total_stock - self.remaining_stock) as f64;

        a * sold.powf(b) + self.base_price
    }

    // ------------------------------------------
    // FINAL PRICE (YOU CAN TUNE THIS MIX)
    // ------------------------------------------
    fn current_price(&self) -> f64 {
        let p1 = self.price_stock_based();
        let p2 = self.price_demand_based();
        let p3 = self.price_bonding_curve();

        // Weighted mix
        (p1 * 0.5) + (p2 * 0.3) + (p3 * 0.2)
    }

    // ------------------------------------------
    // PURCHASE LOGIC
    // ------------------------------------------
    fn purchase(&mut self) {
        if self.remaining_stock == 0 {
            println!("❌ OUT OF STOCK!");
            return;
        }

        self.remaining_stock -= 1;
        self.purchases += 1;

        let now = Instant::now();
        self.last_purchase_time = now;
        self.demand_window.push(now);

        // Remove old timestamps beyond 30s
        self.demand_window.retain(|t| now.duration_since(*t).as_secs() <= 30);

        println!(
            "🛒 Purchased 1 item. Remaining: {} | New Price: {:.2} KES",
            self.remaining_stock,
            self.current_price()
        );
    }
}

fn main() {
    let mut vm = VendingMachine::new(100.0, 10);

    println!("🚀 Smart Vending Machine Started");
    println!("Base price = 100 KES");
    println!("-------------------------------------\n");

    for _ in 0..5 {
        vm.purchase();
        std::thread::sleep(Duration::from_millis(800)); // simulate users buying
    }

    println!("\n📊 Final Machine State:");
    println!("{:#?}", vm);
}
