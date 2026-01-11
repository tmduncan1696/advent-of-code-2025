use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub trait Distance {
    fn sq_distance(&self, other: &Self) -> usize;
}

pub fn get_distances<T>(vals: &Vec<T>) -> HashMap<(T, T), usize> 
where T:
    Distance + Hash + Eq + Clone
{
    let mut pairs = Vec::new();

    for (i, x) in vals.iter().enumerate() {
        for y in vals[(i+1)..].iter() {
            if x != y {
                pairs.push((x.clone(), y.clone()));
            }
        }
    }

    let mut out = HashMap::new();
    for pair in pairs {
        let d = pair.0.sq_distance(&pair.1);
        out.insert(pair, d);
    }

    out
}
