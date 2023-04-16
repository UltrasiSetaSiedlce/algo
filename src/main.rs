use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct PalettSegment {
    idx: usize,
    len: usize,
}

impl PalettSegment {
    fn new_palett(width: usize, height: usize) -> Vec<Vec<PalettSegment>> {
        (0..height)
            .map(|_| vec![PalettSegment { idx: 0, len: width }])
            .collect_vec()
    }
}

#[derive(Clone, Copy, Debug)]
struct Box {
    id: usize,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
struct PackingPlan {
    plan: HashMap<usize, (usize, usize)>,
    wasted: usize,
}

impl PackingPlan {
    fn initial() -> PackingPlan {
        PackingPlan {
            plan: HashMap::new(),
            wasted: usize::MAX,
        }
    }
}

impl Box {
    fn new(id: usize, width: usize, height: usize) -> Box {
        Box { id, width, height }
    }
}

fn main() {
    let mut ids = 1..;
    let palett = PalettSegment::new_palett(15, 15);
    let boxes = vec![
        Box::new(ids.next().unwrap(), 6, 6),
        Box::new(ids.next().unwrap(), 2, 3),
        Box::new(ids.next().unwrap(), 4, 5),
        Box::new(ids.next().unwrap(), 1, 1),
        Box::new(ids.next().unwrap(), 2, 3),
        Box::new(ids.next().unwrap(), 6, 6),
        Box::new(ids.next().unwrap(), 2, 3),
        Box::new(ids.next().unwrap(), 4, 5),
        Box::new(ids.next().unwrap(), 1, 1),
        Box::new(ids.next().unwrap(), 2, 3),
        Box::new(ids.next().unwrap(), 2, 3),
    ];
    println!("{:?}", fit(palett, boxes));
}

fn fit(palett: Vec<Vec<PalettSegment>>, boxes: Vec<Box>) -> Option<PackingPlan> {
    fit_impl(palett, boxes, 0, 0, usize::MAX)
}

fn fit_impl(
    palett: Vec<Vec<PalettSegment>>,
    boxes: Vec<Box>,
    wasted: usize,
    z: usize,
    mut least_wasted: usize,
) -> Option<PackingPlan> {
    if wasted >= least_wasted {
        return None;
    } else if boxes.len() == 0 {
        println!("found! {}", wasted);
        return Some(PackingPlan {
            wasted,
            plan: HashMap::new(),
        });
    }
    let mut best_plan = PackingPlan::initial();
    for (i, boks) in boxes.iter().enumerate() {
        let mut x = match palett.first().and_then(|row| row.first()) {
            Some(seg) => seg.idx,
            None => continue,
        };
        let mut palett_copy = palett.clone();
        let palett_len_pre = palett_copy.len();
        let mut wasted_here = wasted;
        let mut result = fit_box(&palett_copy, boks);
        while let None = result {
            let mut seg = match palett_copy.first_mut().and_then(|row| row.first_mut()) {
                Some(seg) => seg,
                None => break,
            };
            seg.len -= 1;
            seg.idx += 1;
            x = seg.idx;
            wasted_here += 1;
            if wasted_here >= least_wasted {
                break;
            }
            if seg.len == 0 {
                palett_copy[0].remove(0);
                if palett_copy[0].is_empty() {
                    palett_copy.remove(0);
                    x = match palett_copy.first().and_then(|row| row.first()) {
                        Some(seg) => seg.idx,
                        None => break,
                    };
                }
            }
            result = fit_box(&palett_copy, boks);
        }
        if let None = result {
            continue;
        }
        let new_palett = result.unwrap();
        let new_z = z + (palett_len_pre - new_palett.len());
        let mut new_boxes = Vec::with_capacity(boxes.len() - 1);
        new_boxes.extend_from_slice(&boxes[0..i]);
        new_boxes.extend_from_slice(&boxes[i + 1..]);
        let mut new_plan = match fit_impl(new_palett, new_boxes, wasted_here, new_z, least_wasted) {
            Some(wasted) => wasted,
            None => continue,
        };
        if new_plan.wasted < best_plan.wasted {
            new_plan.plan.insert(boks.id, (x, new_z));
            least_wasted = new_plan.wasted;
            best_plan = new_plan;
        }
    }
    if best_plan.plan.is_empty() {
        None
    } else {
        Some(best_plan)
    }
}

fn fit_box(palett: &Vec<Vec<PalettSegment>>, boks: &Box) -> Option<Vec<Vec<PalettSegment>>> {
    if palett.len() < boks.height {
        return None;
    }
    let mut result: Vec<Vec<PalettSegment>> = Vec::new();
    let first_seg_idx = palett[0][0].idx;
    for (row_i, row) in palett.iter().enumerate() {
        let mut new_row: Vec<PalettSegment> = Vec::new();
        for seg in row {
            if row_i >= boks.height {
                new_row.push(*seg);
            } else if seg.idx <= first_seg_idx {
                let pre_seg = PalettSegment {
                    idx: seg.idx,
                    len: first_seg_idx - seg.idx,
                };
                if pre_seg.len > 0 {
                    new_row.push(pre_seg);
                }
                if (seg.len as i32) - (pre_seg.len as i32) < (boks.width as i32) {
                    return None;
                } else {
                    let post_seg = PalettSegment {
                        idx: first_seg_idx + boks.width,
                        len: seg.len - pre_seg.len - boks.width,
                    };
                    if post_seg.len > 0 {
                        new_row.push(post_seg);
                    }
                }
            } else {
                new_row.push(*seg)
            }
        }
        if !new_row.is_empty() {
            result.push(new_row)
        }
    }
    // println!("POST: {:?}", result);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut ids = 1..;
        let palett = PalettSegment::new_palett(15, 15);
        let boxes = vec![
            Box::new(ids.next().unwrap(), 6, 6),
            Box::new(ids.next().unwrap(), 2, 3),
            Box::new(ids.next().unwrap(), 4, 5),
            Box::new(ids.next().unwrap(), 1, 1),
            Box::new(ids.next().unwrap(), 2, 3),
            Box::new(ids.next().unwrap(), 6, 6),
            Box::new(ids.next().unwrap(), 2, 3),
            Box::new(ids.next().unwrap(), 4, 5),
            Box::new(ids.next().unwrap(), 1, 1),
            Box::new(ids.next().unwrap(), 2, 3),
            Box::new(ids.next().unwrap(), 2, 3),
        ];
        fit(palett, boxes);
    }
}
