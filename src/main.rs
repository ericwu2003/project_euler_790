use rayon::prelude::*;

const BOARD_SIZE: i32 = 50515093;
const S_0: i32 = 290797;

const T_MAX: usize = 100_000; // will later be changed to 10^5
const S_ARR: [i32; (T_MAX * 4) as usize] = compute_s_arr();

const fn compute_s_arr() -> [i32; (T_MAX * 4) as usize] {
    let mut s_arr = [0; (T_MAX * 4) as usize];
    let mut s_curr: i64 = S_0 as i64;
    let mut i = 0;
    while i < (T_MAX * 4) as usize {
        s_arr[i] = s_curr as i32;
        s_curr = (s_curr * s_curr) % (BOARD_SIZE as i64);
        i += 1;
    }

    s_arr
}

#[derive(Debug)]
struct Row {
    // a row containing rows with y coordinate from min_y to max_y (half inclusive)
    pub min_y: i32, // inclusive
    pub max_y: i32, // exclusive
}

fn main() {
    let mut rows: Vec<Row> = Vec::with_capacity(T_MAX * 2 + 1);

    let mut division_points = Vec::with_capacity(T_MAX * 2);

    for t in 1..=T_MAX {
        let mut y_min = S_ARR[4 * t - 2];
        let mut y_max = S_ARR[4 * t - 1];

        if y_max < y_min {
            std::mem::swap(&mut y_min, &mut y_max);
        }

        division_points.push(y_min);
        division_points.push(y_max + 1);
    }
    division_points.sort();

    rows.push(Row {
        min_y: 0,
        max_y: *division_points.get(0).unwrap(),
    });
    for i in 1..division_points.len() {
        rows.push(Row {
            min_y: *division_points.get(i - 1).unwrap(),
            max_y: *division_points.get(i).unwrap(),
        });
    }
    rows.push(Row {
        min_y: *division_points.get(division_points.len() - 1).unwrap(),
        max_y: BOARD_SIZE,
    });

    // dbg!(&rows[0..50]);
    // dbg!(&rows.len());

    // let mut grand_total: i64 = 0;

    // for (index, row) in rows.iter().enumerate() {
    //     grand_total += (row.max_y - row.min_y) as i64
    //         * calculate_row_clock_hands_sum((row.min_y + row.max_y) / 2);
    // }

    let grand_total = rows
        .into_par_iter()
        .fold(
            || 0i64,
            |acc, row| {
                acc + (row.max_y - row.min_y) as i64
                    * calculate_row_clock_hands_sum((row.min_y + row.max_y) / 2)
            },
        )
        .sum::<i64>();
    // dbg!(division_points);

    println!("the grand total is {}", grand_total);
}

#[derive(Debug)]
struct Region {
    // also half-inclusive
    count: i32,
    start: i32, // inclusive
    end: i32,   // exclusive
}

fn calculate_row_clock_hands_sum(y_pos: i32) -> i64 {
    let mut regions = vec![Region {
        count: 0,
        start: 0,
        end: BOARD_SIZE,
    }];

    for t in 1..=T_MAX {
        let mut x_min = S_ARR[4 * t - 4];
        let mut x_max = S_ARR[4 * t - 3];
        let mut y_min = S_ARR[4 * t - 2];
        let mut y_max = S_ARR[4 * t - 1];
        if y_max < y_min {
            std::mem::swap(&mut y_min, &mut y_max);
        }
        if x_max < x_min {
            std::mem::swap(&mut x_min, &mut x_max);
        }

        if !(y_min <= y_pos && y_pos <= y_max) {
            continue;
        }

        // update all regions between x_min and x_max
        let mut i = 0;
        while i < regions.len() {
            let r = regions.get_mut(i).unwrap();

            if r.start >= x_min && r.end <= x_max + 1 {
                // case where region lies entirely within update region
                r.count += 1;
            } else if r.end > x_max + 1 && r.start < x_min {
                // split into 3 regions
                let r = regions.swap_remove(i);
                regions.push(Region {
                    count: r.count,
                    start: r.start,
                    end: x_min,
                });
                regions.push(Region {
                    count: r.count,
                    start: x_max + 1,
                    end: r.end,
                });
                regions.push(Region {
                    count: r.count + 1,
                    start: x_min,
                    end: x_max + 1,
                });
                let last_index = regions.len() - 1;
                regions.swap(i, last_index);
                break; // we can break here because no other regions will be affected.
            } else if r.start <= x_max && r.end > x_max + 1 {
                // split into 2 regions
                let r = regions.swap_remove(i);
                regions.push(Region {
                    count: r.count,
                    start: x_max + 1,
                    end: r.end,
                });
                regions.push(Region {
                    count: r.count + 1,
                    start: r.start,
                    end: x_max + 1,
                });
                let last_index = regions.len() - 1;
                regions.swap(i, last_index);
            } else if r.end > x_min && r.start < x_min {
                // split into 2 regions
                let r = regions.swap_remove(i);
                regions.push(Region {
                    count: r.count,
                    start: r.start,
                    end: x_min,
                });
                regions.push(Region {
                    count: r.count + 1,
                    start: x_min,
                    end: r.end,
                });
                let last_index = regions.len() - 1;
                regions.swap(i, last_index);
            } else {
                // no overlap, do nothing.
            }
            i += 1;
        }
        // regions.sort_by_key(|region| region.start);
        // dbg!(&regions);
    }
    let mut total: i64 = 0;

    for region in &regions {
        let mut clock_num = region.count % 12;
        if clock_num == 0 {
            clock_num = 12;
        }
        total += ((region.end - region.start) * clock_num) as i64;
    }

    // dbg!(&regions);
    // println!("the sum at {} is {}", y_pos, total);
    total
}