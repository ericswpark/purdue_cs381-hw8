use anyhow::{anyhow, Result};

use serde::Deserialize;

fn extract_possible_min_duration_order(orders: &mut Vec<(u32, u32, u32)>, current_time: u32) -> Option<(u32, u32, u32)> {
    if let Some(&min_value) = orders.iter().filter(|&x| x.0 <= current_time).min_by_key(|x| x.1) {
        let index = orders.iter().position(|&x| x == min_value).unwrap();
        return Some(orders.remove(index));
    }
    None
}


pub fn starbucks_scheduler(t: &[u32], d: &[u32]) -> Result<u32> {
    if t.len() != d.len() {
        return Err(anyhow!("Start time and duration time arrays do not match in length!"));
    }

    // Create zipped array of orders
    // Start time, duration, and completed-at time
    let mut pending_orders: Vec<(u32, u32, u32)> = Vec::new();
    for (start_time, duration) in t.iter().zip(d.iter()) {
        pending_orders.push((*start_time, *duration, 0));
    }

    let mut completed_orders = Vec::new();
    let mut current_time = 0;

    while pending_orders.len() > 0 {
        // Find order with the lowest duration left that has been assigned at current time
        let next_order = extract_possible_min_duration_order(&mut pending_orders, current_time);

        current_time += 1;
        
        // Check if there is actually an order we can process right now
        // (It's possible that there are no orders assigned)
        if let Some(mut next_order) = next_order {
            next_order.1 -= 1;

            if next_order.1 == 0 {
                next_order.2 = current_time;
                completed_orders.push(next_order);
            } else {
                pending_orders.push(next_order);
            }
        }
    }
    
    Ok(completed_orders.iter().map(|x| x.2).sum())
}



// Only used for test harness, silence dead code warning
#[allow(dead_code)]
#[derive(Deserialize)]
struct Q1TestCase {
    name: String,
    t: Vec<u32>,
    d: Vec<u32>,
    result: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starbucks_scheduler_tcs() {
        let tcs_str = include_str!("../q1_test_cases.json");
        let tcs: Vec<Q1TestCase> = serde_json::from_str(&tcs_str).expect("Invalid TC JSON file!");

        for tc in tcs {
            assert_eq!(
                starbucks_scheduler(tc.t.as_slice(), tc.d.as_slice()).unwrap(),
                tc.result,
                "Test case {} failed!",
                tc.name
            );
        }
    }

}
