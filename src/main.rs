use std::cmp::min;

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
    width: usize,
    height: usize,
}

impl Box {
    fn new(width: usize, height: usize) -> Box {
        Box { width, height }
    }
}

fn main() {
    let palett = PalettSegment::new_palett(15, 15);
    let boxes = vec![
        Box::new(6, 6),
        Box::new(2, 3),
        Box::new(4, 5),
        Box::new(1, 1),
        Box::new(2, 3),
        Box::new(6, 6),
        Box::new(2, 3),
        Box::new(4, 5),
        Box::new(1, 1),
        Box::new(2, 3),
        Box::new(2, 3),
        Box::new(4, 5),
        Box::new(4, 5),
        Box::new(2, 3),
    ];
    println!("{}", fit(palett, boxes, 0, usize::MAX));
}

fn fit(
    palett: Vec<Vec<PalettSegment>>,
    boxes: Vec<Box>,
    wasted: usize,
    mut least_wasted_so_far: usize,
) -> usize {
    if boxes.len() == 0 {
        println!("found! {}", wasted);
        return wasted;
    } else if wasted >= least_wasted_so_far {
        return least_wasted_so_far;
    }
    for (i, boks) in boxes.iter().enumerate() {
        let mut palett_copy = palett.clone();
        let mut wasted_here = wasted;
        let mut result = fit_box(&palett_copy, boks);
        while let None = result {
            let mut seg = match palett_copy.first_mut().and_then(|row| row.first_mut()) {
                Some(seg) => seg,
                None => break,
            };
            seg.len -= 1;
            seg.idx += 1;
            wasted_here += 1;
            if wasted_here >= least_wasted_so_far {
                break;
            }
            if seg.len == 0 {
                palett_copy[0].remove(0);
                if palett_copy[0].is_empty() {
                    palett_copy.remove(0);
                }
            }
            result = fit_box(&palett_copy, boks);
        }
        if let None = result {
            continue;
        }
        let mut new_boxes = Vec::with_capacity(boxes.len() - 1);
        new_boxes.extend_from_slice(&boxes[0..i]);
        new_boxes.extend_from_slice(&boxes[i + 1..]);
        wasted_here = fit(result.unwrap(), new_boxes, wasted_here, least_wasted_so_far);
        least_wasted_so_far = min(least_wasted_so_far, wasted_here)
    }
    least_wasted_so_far
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
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let palett = PalettSegment::new_palett(10, 10);
        let boxes = vec![
            Box::new(2, 3),
            Box::new(4, 5),
            Box::new(1, 1),
            Box::new(2, 3),
            Box::new(2, 3),
            Box::new(4, 5),
            Box::new(1, 1),
            Box::new(2, 3),
        ];
        fit(palett, boxes, 0, usize::MAX);
    }
}
