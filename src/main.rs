extern crate matrix;

use matrix::prelude::*;

type Point = (i32, i32);

#[derive(Clone, Debug)]
struct Node {
    point: Point,
    children: Vec<Node>,
}

fn moves() -> Vec<Point> {
    vec![
        (2, 1),
        (-2, 1),
        (2, -1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ]
}

fn get_pos_sol(matrix: &Conventional<i32>, pos: &Point) -> Vec<Point> {
    let rows: i32 = matrix.rows as i32;
    let cols: i32 = matrix.columns as i32;
    moves().iter().fold(vec![], |acc, (i, j)| {
        let inw = pos.0 + i;
        let jnw = pos.1 + j;
        if inw >= 0 && inw < rows && jnw >= 0 && jnw < cols {
            let mut res = Vec::with_capacity(acc.len());
            res.extend_from_slice(&acc);
            res.push((inw, jnw));
            res
        } else {
            acc
        }
    })
}

fn search_for_pos(matrix: &Conventional<i32>, pos: &Point, depth: i32, max_depth: i32) -> Node {
    if depth >= max_depth {
        return Node {
            point: pos.clone(),
            children: vec![],
        };
    }
    let solutions = get_pos_sol(matrix, pos);
    let children: Vec<Node> = solutions.iter().fold(vec![], |acc, next| {
        let inner_children = search_for_pos(matrix, next, depth + 1, max_depth);
        let mut res = Vec::with_capacity(acc.len());
        res.extend_from_slice(&acc);
        res.push(inner_children);
        res
    });

    Node {
        point: pos.clone(),
        children: children,
    }
}

fn rec_build(matrix: &Conventional<i32>, res: &mut Vec<Vec<i32>>, elem: Node, acc: &Vec<i32>) {
    let mut copied = Vec::with_capacity(acc.len());
    copied.extend_from_slice(acc);
    let posi = (elem.point.0 as usize, elem.point.1 as usize);
    let val = matrix[posi];
    copied.push(val);
    for celem in elem.children {
        rec_build(matrix, res, celem, &copied);
    }
    res.push(copied);
}

fn show_solutions(
    matrix: &Conventional<i32>,
    pos: &Point,
    solutions: Node,
    steps: i32,
) -> Vec<Vec<i32>> {
    let posi = (pos.0 as usize, pos.1 as usize);
    println!("solutions for {}", matrix[posi]);
    let mut res: Vec<Vec<i32>> = vec![];
    rec_build(matrix, &mut res, solutions, &vec![]);
    let filtered = res
        .iter()
        .cloned()
        .filter(|elem| elem.len() as i32 == steps)
        .collect();
    for sol in &filtered {
        println!("-> {:?}", sol);
    }
    filtered
}

fn main() {
    let n = 5;
    let m = 6;
    let nms = n * m + 1;
    let nums: Vec<i32> = (1..nms).collect();
    let matrix = Conventional::from_slice((n as usize, m as usize), &nums);
    let step = 9;
    let res = search_for_pos(&matrix, &(0, 0), 0, step);
    show_solutions(&matrix, &(0, 0), res, step);
}
