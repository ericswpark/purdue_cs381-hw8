use anyhow::{anyhow, Result};
use pathfinding::matrix::Matrix;
use pathfinding::prelude::kuhn_munkres;
use serde::Deserialize;
use std::cmp::{max, min};

pub fn vip_scheduler(s: &[u32], e: &[u32]) -> Result<u32> {
    if s.len() != e.len() {
        return Err(anyhow!("Start and end time arrays do not match in length!"));
    }

    // Create zipped sorted array of VIP times
    let mut vip_times: Vec<(u32, u32)> = Vec::new();
    for (start_time, end_time) in s.iter().zip(e.iter()) {
        vip_times.push((*start_time, *end_time));
    }
    vip_times.sort_by_key(|x| x.0);

    let mut performance_count = 0;
    let mut overlap_boundary = vip_times[0];

    let mut skip_until: Option<usize> = None;

    for (index, _vip_time) in vip_times.iter().enumerate() {
        // Workaround since we cannot rewrite the index
        if let Some(i) = skip_until {
            if index < i {
                continue;
            }
        }

        let mut next_index = index + 1;

        while next_index < vip_times.len() {
            let next_element = &vip_times[next_index];

            if (overlap_boundary.0..=overlap_boundary.1).contains(&next_element.0)
                || (overlap_boundary.0..=overlap_boundary.1).contains(&next_element.1)
            {
                // Set new overlap boundaries
                overlap_boundary.0 = max(overlap_boundary.0, next_element.0);
                overlap_boundary.1 = min(overlap_boundary.1, next_element.1);

                if overlap_boundary.0 > overlap_boundary.1 {
                    break;
                }
                next_index += 1;
            } else {
                break;
            }
        }

        performance_count += 1;
        skip_until = Some(next_index);
        if next_index < vip_times.len() {
            overlap_boundary = vip_times[next_index];
        }
    }

    Ok(performance_count)
}

pub fn homework_max_points(p: &[u32], t: &[u32], d: &[u32]) -> Result<u32> {
    if p.len() != t.len() || t.len() != d.len() {
        return Err(anyhow!("Provided arrays do not match in length!"));
    }

    for (index, start_date) in t.iter().enumerate() {
        if start_date > &d[index] {
            return Err(anyhow!("Homework assignment {} has start date {}, which is past the due date {}. This is not allowed!", index + 1, start_date, &d[index]));
        }
    }

    let n = p.len();
    let max_days = *max(t.iter().max().unwrap(), d.iter().max().unwrap());
    let cap = 100;

    if n >= cap || max_days >= cap as u32 {
        return Err(anyhow!("Too many homework assignments or too high of a day value. For performance reasons please keep your input smaller. Sorry!"));
    }

    let mut hw_matrix = vec![vec![0; max_days as usize]; n];

    for i in 0..n {
        for day in t[i]..=d[i] {
            hw_matrix[i][(day - 1) as usize] = p[i] as i32;
        }
    }

    if n > max_days as usize {
        hw_matrix = transpose(hw_matrix);
    }

    let weights = Matrix::from_rows(hw_matrix)?;
    let (points, _assignments) = kuhn_munkres(&weights);

    Ok(points as u32)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

// Only used for test harness, silence dead code warning
#[allow(dead_code)]
#[derive(Deserialize)]
struct Q2TestCase {
    name: String,
    s: Vec<u32>,
    e: Vec<u32>,
    result: u32,
}

// Only used for test harness, silence dead code warning
#[allow(dead_code)]
#[derive(Deserialize)]
struct Q4TestCase {
    name: String,
    p: Vec<u32>,
    t: Vec<u32>,
    d: Vec<u32>,
    result: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vip_scheduler_tcs() {
        let tcs_str = include_str!("../q2_test_cases.json");
        let tcs: Vec<Q2TestCase> = serde_json::from_str(&tcs_str).expect("Invalid TC JSON file!");

        for tc in tcs {
            assert_eq!(
                vip_scheduler(tc.s.as_slice(), tc.e.as_slice()).unwrap(),
                tc.result,
                "Test case {} failed!",
                tc.name
            );
        }
    }

    #[test]
    fn test_homework_max_points_tcs() {
        let tcs_str = include_str!("../q4_test_cases.json");
        let tcs: Vec<Q4TestCase> = serde_json::from_str(&tcs_str).expect("Invalid TC JSON file!");

        for tc in tcs {
            assert_eq!(
                homework_max_points(tc.p.as_slice(), tc.t.as_slice(), tc.d.as_slice()).unwrap(),
                tc.result,
                "Test case {} failed!",
                tc.name
            );
        }
    }
    
    #[test]
    fn test_homework_max_points_invalid_start_day() {
        assert!(homework_max_points(&[50, 80], &[1, 4], &[2, 2]).is_err())
    }
}
