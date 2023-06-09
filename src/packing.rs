use itertools::Itertools;

use std::{
    collections::HashMap,
    iter::repeat,
    time::{Duration, Instant},
};

use crate::schema::{Box, FilledPalett, PackingPlan};

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

#[derive(Debug, Clone)]
struct SemiPackingPlan {
    plan: HashMap<usize, (usize, usize, usize)>,
    wasted: usize,
}

impl SemiPackingPlan {
    fn initial() -> SemiPackingPlan {
        SemiPackingPlan {
            plan: HashMap::new(),
            wasted: usize::MAX,
        }
    }
}

pub fn fit(
    (dx, dy, dz): (usize, usize, usize),
    palettes_n: usize,
    mut boxes: Vec<Box>,
    timeout: Duration,
) -> PackingPlan {
    boxes.sort_by(|b1, b2| b2.weight.cmp(&b1.weight));
    let semi = fit_impl(
        (dx, dz),
        PalettSegment::new_palett(dx, dz),
        boxes,
        0,
        0,
        0,
        usize::MAX,
        Instant::now() + Duration::from_secs(10),
    )
    .unwrap();
    println!("SEMI: {:?}", semi);
    let layers_n = semi.plan.values().map(|(_, y, _)| y).max().unwrap() + 1;
    if layers_n > palettes_n * dy {
        panic!("pizda");
    };
    let mut palettes = repeat(FilledPalett::new()).take(palettes_n).collect_vec();
    let layers = semi
        .plan
        .iter()
        .sorted_by(|(_, (_, y1, _)), (_, (_, y2, _))| y1.cmp(y2))
        .group_by(|(_, (_, y, _))| y);
    layers
        .into_iter()
        .sorted_by(|(y1, _), (y2, _)| y1.cmp(y2))
        .for_each(|(y, group)| {
            let idx = y % palettes.len();
            let mut palette = &mut palettes[idx];
            for (i, (x, _, z)) in group {
                let pos = (*x, palette.dy, *z);
                palette.boxes.insert(*i, pos);
            }
            palette.dy += 1;
        });
    PackingPlan { palettes }
}

fn fit_impl(
    palett_size @ (width, height): (usize, usize),
    palett: Vec<Vec<PalettSegment>>,
    boxes: Vec<Box>,
    wasted: usize,
    z: usize,
    y: usize,
    mut least_wasted: usize,
    stop_time: Instant,
) -> Option<SemiPackingPlan> {
    if wasted >= least_wasted || Instant::now() > stop_time {
        return None;
    } else if boxes.len() == 0 {
        println!("found! {}", wasted);
        return Some(SemiPackingPlan {
            wasted,
            plan: HashMap::new(),
        });
    }
    let mut best_plan = SemiPackingPlan::initial();
    for (i, boks) in boxes.iter().enumerate() {
        let mut new_y = y;
        let mut new_z = z;
        let mut palett_copy = palett.clone();
        let mut x = match palett_copy.first().and_then(|row| row.first()) {
            Some(seg) => seg.idx,
            None => {
                palett_copy = PalettSegment::new_palett(width, height);
                new_y += 1;
                new_z = 0;
                0
            }
        };
        let mut wasted_here = wasted;
        let mut result = fit_box(&palett_copy, &boks);
        while let None = result {
            let mut seg = match palett_copy.first_mut().and_then(|row| row.first_mut()) {
                Some(seg) => seg,
                None => {
                    palett_copy = PalettSegment::new_palett(width, height);
                    new_y += 1;
                    new_z = 0;
                    &mut palett_copy[0][0]
                }
            };
            seg.len -= 1;
            seg.idx += 1;
            x = seg.idx;
            wasted_here += 1;
            if wasted_here >= least_wasted || Instant::now() > stop_time{
                break;
            }
            if seg.len == 0 {
                palett_copy[0].remove(0);
                if palett_copy[0].is_empty() {
                    palett_copy.remove(0);
                    new_z += 1;
                    x = match palett_copy.first().and_then(|row| row.first()) {
                        Some(seg) => seg.idx,
                        None => {
                            palett_copy = PalettSegment::new_palett(width, height);
                            new_y += 1;
                            new_z = 0;
                            0
                        }
                    };
                }
            }
            result = fit_box(&palett_copy, &boks);
        }
        if let None = result {
            continue;
        }
        let (new_palett, skipped) = result.unwrap();
        let mut new_boxes = Vec::with_capacity(boxes.len() - 1);
        new_boxes.extend_from_slice(&boxes[0..i]);
        new_boxes.extend_from_slice(&boxes[i + 1..]);
        let mut new_plan = match fit_impl(
            palett_size,
            new_palett,
            new_boxes,
            wasted_here,
            new_z + skipped,
            new_y,
            least_wasted,
            stop_time,
        ) {
            Some(wasted) => wasted,
            None => continue,
        };
        if new_plan.wasted < best_plan.wasted {
            new_plan.plan.insert(boks.id, (x, new_y, new_z));
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

fn fit_box(palett: &Vec<Vec<PalettSegment>>, boks: &Box) -> Option<(Vec<Vec<PalettSegment>>, usize)> {
    if palett.len() < boks.dz {
        return None;
    }
    // println!("PRE: {:?}", palett);
    let mut result: Vec<Vec<PalettSegment>> = Vec::new();
    let first_seg_idx = palett[0][0].idx;
    let mut skipped: usize = 0;
    for (row_i, row) in palett.iter().enumerate() {
        let mut new_row: Vec<PalettSegment> = Vec::new();
        for seg in row {
            if row_i >= boks.dz {
                new_row.push(*seg);
            } else if seg.idx <= first_seg_idx {
                let pre_seg = PalettSegment {
                    idx: seg.idx,
                    len: first_seg_idx - seg.idx,
                };
                if pre_seg.len > 0 {
                    new_row.push(pre_seg);
                }
                if (seg.len as i32) - (pre_seg.len as i32) < (boks.dx as i32) {
                    return None;
                } else {
                    let post_seg = PalettSegment {
                        idx: first_seg_idx + boks.dx,
                        len: seg.len - pre_seg.len - boks.dx,
                    };
                    if post_seg.len > 0 {
                        new_row.push(post_seg);
                    }
                }
            } else {
                new_row.push(*seg);
            }
        }
        if !new_row.is_empty() {
            result.push(new_row)
        } else {
            skipped += 1;
        }
    }
    // println!("POST: {:?}", result);
    Some((result, skipped))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{packing::fit, schema::Box};

    fn make_box(id: usize, dx: usize, dz: usize) -> Box {
        Box {
            id,
            dx,
            dz,
            weight: 5,
        }
    }

    #[test]
    fn test() {
        let mut ids = 1..;
        let boxes = vec![
            make_box(ids.next().unwrap(), 6, 6),
            make_box(ids.next().unwrap(), 2, 3),
            make_box(ids.next().unwrap(), 4, 5),
            make_box(ids.next().unwrap(), 1, 1),
            make_box(ids.next().unwrap(), 2, 3),
            make_box(ids.next().unwrap(), 6, 6),
            make_box(ids.next().unwrap(), 2, 3),
            make_box(ids.next().unwrap(), 4, 5),
            make_box(ids.next().unwrap(), 1, 1),
            make_box(ids.next().unwrap(), 2, 3),
            make_box(ids.next().unwrap(), 2, 3),
            make_box(ids.next().unwrap(), 7, 15),
            make_box(ids.next().unwrap(), 8, 15),
        ];
        println!("{:?}", fit((15, 10, 15), 4, boxes, Duration::from_secs(5)));
    }
}
