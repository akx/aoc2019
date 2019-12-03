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

fn to_delta(m: &Move) -> Pos {
    match m {
        Move::U(a) => (0, -a),
        Move::L(a) => (-a, 0),
        Move::D(a) => (0, *a),
        Move::R(a) => (*a, 0),
    }
}

fn to_line_segments(start: &Pos, moves: &Vec<Move>) -> Vec<LineSeg> {
    let mut origin = start.clone();
    let mut lines: Vec<LineSeg> = vec![];
    for mv in moves {
        let delta = to_delta(mv);
        let dest = (origin.0 + delta.0, origin.1 + delta.1);
        lines.push((origin, dest));
        origin = dest;
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
    let lines: Vec<Vec<LineSeg>> = movesets
        .iter()
        .map(|mvl| to_line_segments(&(0, 0), &mvl))
        .collect();
    let mut intersections: Vec<Pos> = Vec::new();
    for l1 in &lines[0] {
        for l2 in &lines[1] {
            match intersect(&l1, &l2) {
                Some(p) => {
                    if !(p.0 == 0 && p.1 == 0) {
                        println!("{:?} {:?} = {:?}", l1, l2, p);
                        intersections.push(p);
                    }
                }
                _ => (),
            }
        }
    }
    match intersections.iter().min_by_key(|p| (p.0.abs() + p.1.abs())) {
        Some(p) => println!("=> {:?}", p),
        _ => (),
    }

    Ok(())
}
