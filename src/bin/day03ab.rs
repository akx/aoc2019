use aoc2019::halp;

type CoordType = i32;

#[derive(Debug)]
enum Move {
    U(CoordType),
    D(CoordType),
    L(CoordType),
    R(CoordType),
}
type Pos = (CoordType, CoordType);
type LineSeg = (Pos, Pos);

#[derive(Debug)]
struct CumLineSeg {
    start_steps: i32,
    end_steps: i32,
    this_steps: i32,
    seg: LineSeg,
}

fn to_delta(m: &Move) -> Pos {
    match m {
        Move::U(a) => (0, -a),
        Move::L(a) => (-a, 0),
        Move::D(a) => (0, *a),
        Move::R(a) => (*a, 0),
    }
}

fn manhattan(p: &Pos) -> i32 {
    (p.0).abs() + (p.1).abs()
}

fn manhattan2(a: &Pos, b: &Pos) -> i32 {
    manhattan(&(b.0 - a.0, b.1 - a.1))
}

fn to_line_segments(start: &Pos, moves: &Vec<Move>) -> Vec<CumLineSeg> {
    let mut origin = start.clone();
    let mut lines: Vec<CumLineSeg> = vec![];
    let mut steps: i32 = 0;
    for mv in moves {
        let delta = to_delta(mv);
        let dest = (origin.0 + delta.0, origin.1 + delta.1);
        let this_steps = manhattan(&delta);
        lines.push(CumLineSeg {
            start_steps: steps,
            end_steps: steps + this_steps,
            this_steps,
            seg: (origin, dest),
        });
        origin = dest;
        steps += this_steps;
    }
    lines
}

fn parse_moves(line: &String) -> Vec<Move> {
    line.trim()
        .split(",")
        .map(|atom| {
            let delta = atom[1..].parse::<CoordType>().unwrap();
            match atom.chars().nth(0).unwrap() {
                'U' => Move::U(delta),
                'L' => Move::L(delta),
                'R' => Move::R(delta),
                'D' => Move::D(delta),
                _ => panic!("what is {}", atom),
            }
        })
        .collect()
}

fn intersect(la: &LineSeg, lb: &LineSeg) -> Option<Pos> {
    let p1 = la.1;
    let p0 = la.0;
    let p2 = lb.0;
    let p3 = lb.1;
    let s10_x = p1.0 - p0.0;
    let s10_y = p1.1 - p0.1;
    let s32_x = p3.0 - p2.0;
    let s32_y = p3.1 - p2.1;

    let denom = s10_x * s32_y - s32_x * s10_y;

    if denom == 0 {
        return None;
    }
    let denom_is_positive = denom > 0;

    let s02_x = p0.0 - p2.0;
    let s02_y = p0.1 - p2.1;

    let s_numer = s10_x * s02_y - s10_y * s02_x;

    if (s_numer < 0) == denom_is_positive {
        return None;
    }

    let t_numer = s32_x * s02_y - s32_y * s02_x;

    if (t_numer < 0) == denom_is_positive {
        return None; // no collision
    }

    if (s_numer > denom) == denom_is_positive || (t_numer > denom) == denom_is_positive {
        return None; // no collision
    }

    // collision detected

    let t: f32 = t_numer as f32 / denom as f32;
    let tx = p0.0 as f32 + (t * s10_x as f32);
    let ty = p0.1 as f32 + (t * s10_y as f32);

    return Some((tx as i32, ty as i32));
}

fn main() -> Result<(), std::io::Error> {
    let flines = halp::lines_from_file("inputs/03.txt");
    let movesets: Vec<Vec<Move>> = flines.iter().map(|l| parse_moves(&l)).collect();
    let lines: Vec<Vec<CumLineSeg>> = movesets
        .iter()
        .map(|mvl| to_line_segments(&(0, 0), &mvl))
        .collect();
    //println!("{:#?}", lines);
    let mut intersections: Vec<(i32, i32, Pos)> = Vec::new();
    for l1 in &lines[0] {
        for l2 in &lines[1] {
            match intersect(&l1.seg, &l2.seg) {
                Some(p) => {
                    if !(p.0 == 0 && p.1 == 0) {
                        let st1 = manhattan2(&l1.seg.0, &p);
                        let st2 = manhattan2(&l2.seg.0, &p);
                        //println!("p1: {:?} -> {:?} = {}", l1, p, st1);
                        //println!("p2: {:?} -> {:?} = {}", l2, p, st2);
                        intersections.push((l1.start_steps + st1, l2.start_steps + st2, p));
                    }
                }
                _ => (),
            }
        }
    }
    match intersections.iter().min_by_key(|t| manhattan(&t.2)) {
        Some(p) => println!("a => {:?} (md = {})", p, manhattan(&p.2)),
        _ => (),
    }
    match intersections.iter().min_by_key(|t| t.0 + t.1) {
        Some(p) => println!("b => {:?} (sum = {})", p, p.0 + p.1),
        _ => (),
    }

    Ok(())
}
