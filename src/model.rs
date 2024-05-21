#![allow(dead_code)]

use std::collections::HashSet;

use chrono::NaiveDate;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Out of stock")]
    OutOfStock,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Quantity(i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Sku(String);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Reference(String);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct OrderLine {
    orderid: String,
    sku: Sku,
    qty: Quantity,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Batch {
    ref_: Reference,
    sku: Sku,
    purchased_quantity: Quantity,
    eta: Option<NaiveDate>,
    allocations: HashSet<OrderLine>,
}

impl Ord for Batch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eta.is_none() {
            return std::cmp::Ordering::Greater;
        }
        if other.eta.is_none() {
            return std::cmp::Ordering::Greater;
        }
        self.eta
            .expect("ETA should not be None")
            .cmp(&other.eta.expect("ETA should not be None"))
    }
}

impl PartialOrd for Batch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Batch {
    fn new(ref_: Reference, sku: Sku, qty: Quantity, eta: Option<NaiveDate>) -> Self {
        Self {
            ref_,
            sku,
            purchased_quantity: qty,
            eta,
            allocations: HashSet::new(),
        }
    }
    fn allocate(&mut self, line: OrderLine) {
        if self.can_allocate(&line) {
            self.allocations.insert(line);
        }
    }

    fn deallocate(&mut self, line: OrderLine) {
        if self.allocations.contains(&line) {
            self.allocations.remove(&line);
        }
    }

    fn allocated_quantity(&self) -> i32 {
        self.allocations.iter().map(|line| line.qty.0).sum()
    }

    fn available_quantity(&self) -> i32 {
        self.purchased_quantity.0 - self.allocated_quantity()
    }

    fn can_allocate(&self, line: &OrderLine) -> bool {
        self.sku.0 == line.sku.0 && self.available_quantity() >= line.qty.0
    }
}

pub fn allocate(line: OrderLine, batches: &mut [Batch]) -> Result<Option<String>, DomainError> {
    batches.sort();
    let mut refid: Option<String> = None;
    for batch in batches.iter_mut() {
        if batch.can_allocate(&line) {
            batch.allocate(line.clone());
            refid = Some(batch.ref_.0.to_string());
            break;
        }
    }
    if refid.is_none() {
        return Err(DomainError::OutOfStock);
    }
    Ok(refid)
}
// tests
#[cfg(test)]
mod tests {
    use chrono::Local;

    use super::*;

    fn make_batch_and_line(sku: Sku, batch_qty: i32, line_qty: i32) -> (Batch, OrderLine) {
        (
            Batch {
                ref_: Reference("batch-001".to_string()),
                sku: sku.clone(),
                purchased_quantity: Quantity(batch_qty),
                eta: Some(Local::now().naive_local().date()),
                allocations: HashSet::new(),
            },
            OrderLine {
                orderid: "order-ref".to_string(),
                sku,
                qty: Quantity(line_qty),
            },
        )
    }

    #[test]
    fn test_allocating_to_a_batch_reduces_the_available_quantity() {
        let mut batch = Batch {
            ref_: Reference("batch-001".to_string()),
            sku: Sku("SMALL-TABLE".to_string()),
            purchased_quantity: Quantity(20),
            eta: Some(Local::now().naive_local().date()),
            allocations: HashSet::new(),
        };
        let line = OrderLine {
            orderid: "order-ref".to_string(),
            sku: Sku("SMALL-TABLE".to_string()),
            qty: Quantity(2),
        };
        batch.allocate(line);
        assert_eq!(batch.available_quantity(), 18);
    }

    #[test]
    fn test_can_allocate_if_available_greater_than_required() {
        let (large_batch, small_line) = make_batch_and_line(Sku("ELEGANT-LAMP".to_string()), 20, 2);
        assert!(large_batch.can_allocate(&small_line));
    }

    #[test]
    fn test_cannot_allocate_if_available_smaller_than_required() {
        let (small_batch, large_line) = make_batch_and_line(Sku("ELEGANT-LAMP".to_string()), 2, 20);
        assert!(!small_batch.can_allocate(&large_line));
    }

    #[test]
    fn test_can_allocate_if_available_equal_to_required() {
        let (batch, line) = make_batch_and_line(Sku("ELEGANT-LAMP".to_string()), 2, 2);
        assert!(batch.can_allocate(&line));
    }

    #[test]
    fn test_can_only_deallocate_allocated_lines() {
        let (mut batch, unallocated_line) =
            make_batch_and_line(Sku("DECORATIVE-TRINKET".to_string()), 20, 2);
        batch.deallocate(unallocated_line);
        assert_eq!(batch.available_quantity(), 20);
    }

    #[test]
    fn test_batch_allocation_should_be_idempotent() {
        let (mut batch, line) = make_batch_and_line(Sku("ANGULAR-DESK".to_string()), 20, 2);
        batch.allocate(line.clone());
        batch.allocate(line);
        assert_eq!(batch.available_quantity(), 18);
    }

    #[test]
    fn test_prefers_current_stock_batches_to_shipments() {
        let in_stock = Batch::new(
            Reference("in-stock".to_string()),
            Sku("RETRO-CLOCK".to_string()),
            Quantity(100),
            None,
        );
        let shipment = Batch::new(
            Reference("shipment".to_string()),
            Sku("RETRO-CLOCK".to_string()),
            Quantity(100),
            Some(Local::now().naive_local().date()),
        );
        let line = OrderLine {
            orderid: "oref".to_string(),
            sku: Sku("RETRO-CLOCK".to_string()),
            qty: Quantity(10),
        };
        let expected = shipment.ref_.0.to_string();
        let mut batches = vec![shipment, in_stock];
        let refid = allocate(line, &mut batches).unwrap();
        assert_eq!(expected, refid.unwrap());
        assert_eq!(batches[0].available_quantity(), 90);
        assert_eq!(batches[1].available_quantity(), 100);
    }

    #[test]
    fn test_prefers_earlier_batches() {
        let earliest = Batch::new(
            Reference("speedy".to_string()),
            Sku("MINIMALIST-SPOON".to_string()),
            Quantity(100),
            Some(NaiveDate::from_ymd_opt(2024, 5, 21).unwrap()),
        );
        let medium = Batch::new(
            Reference("normal".to_string()),
            Sku("MINIMALIST-SPOON".to_string()),
            Quantity(100),
            Some(NaiveDate::from_ymd_opt(2024, 5, 22).unwrap()),
        );
        let latest = Batch::new(
            Reference("slow".to_string()),
            Sku("MINIMALIST-SPOON".to_string()),
            Quantity(100),
            Some(NaiveDate::from_ymd_opt(2024, 5, 26).unwrap()),
        );
        let line = OrderLine {
            orderid: "order-001".to_string(),
            sku: Sku("MINIMALIST-SPOON".to_string()),
            qty: Quantity(10),
        };
        let expected = earliest.ref_.0.to_string();
        let mut batches = vec![medium, earliest, latest];
        let refid = allocate(line, &mut batches).unwrap();
        assert_eq!(expected, refid.unwrap());
        assert_eq!(batches[0].available_quantity(), 90);
        assert_eq!(batches[1].available_quantity(), 100);
    }

    #[test]
    fn test_raises_out_of_stock_exception_if_cannot_allocate() {
        let batch = Batch::new(
            Reference("batch-001".to_string()),
            Sku("SMALL-FORK".to_string()),
            Quantity(10),
            None,
        );
        let line = OrderLine {
            orderid: "order-001".to_string(),
            sku: Sku("SMALL-KNIFE".to_string()),
            qty: Quantity(10),
        };
        let mut batches = vec![batch];
        let result = allocate(line, &mut batches);
        assert!(result.is_err());
    }
}
