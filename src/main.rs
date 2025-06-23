use colored::Colorize;
use rand::Rng;
use std::cmp::PartialEq;
use std::thread::sleep;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq)]
enum MapPixel {
    Wall,
    Highlighted,
    DeadEnd,
    Path,
    Air,
    Checked,
}

#[derive(Clone)]
struct Node {
    x: usize,
    y: usize,
    from: Option<Box<Node>>,
}

const FIELD_SIZE: usize = 64;

fn main() {
    let mut field = [[MapPixel::Air; FIELD_SIZE]; FIELD_SIZE];

    for x in 0..field.len() {
        for y in 0..field[x].len() {
            if x == 0 || y == 0 || y == field[x].len() - 1 || x == field.len() - 1 {
                field[x][y] = MapPixel::Wall;
            }
        }
    }

    let mut closed_list: Vec<(usize, usize, usize)> = Vec::new();
    let mut open_list: Vec<(Node, usize)> = Vec::new();

    let mut rng = rand::rng();

    let player_x: usize = rng.random_range(0..field.len());
    let player_y: usize = rng.random_range(0..field.len());

    let goal_x = rng.random_range(0..field.len());
    let goal_y = rng.random_range(0..field.len());

    for x in 0..field.len() {
        for y in 0..field[x].len() {
            if rng.random::<f32>() < 0.5 {
                field[x][y] = MapPixel::Wall;
            }
        }
    }

    field[player_x][player_y] = MapPixel::Air;
    field[goal_x][goal_y] = MapPixel::Air;

    let start_node = Node {
        x: player_x,
        y: player_y,
        from: None,
    };
    open_list.push((start_node, 0));

    loop {
        let consumed_node_index = open_list
            .iter()
            .enumerate()
            .min_by_key(|(_, (node, b))| *b + heuristics((node.x, node.y), (goal_x, goal_y)))
            .map(|(index, _)| index);

        let (consumed_node, cost) = match consumed_node_index {
            Some(index) => open_list.remove(index),
            None => {
                println!("{}", "Can't reach".red());
                break;
            }
        };
        if consumed_node.x == goal_x && consumed_node.y == goal_y {
            let mut steps = 0;
            let mut node = consumed_node;
            while let Some(from_node) = node.from {
                node = *from_node;
                field[node.x][node.y] = MapPixel::Path;
                steps += 1;
            }
            print_matrix(
                (player_x, player_y),
                (goal_x, goal_y),
                (node.x, node.y),
                &field,
            );
            println!("{}", "Goal reached".green());
            println!("Steps: {}", steps);
            break;
        }
        closed_list.push((consumed_node.x, consumed_node.y, cost));
        let mut dead_end = true;
        field[consumed_node.x][consumed_node.y] = MapPixel::Checked;
        for (x, y) in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ] {
            let new_cost = ((isize::abs(x) + isize::abs(y)) + cost as isize) as usize;
            let nx = consumed_node.x as isize + x;
            let ny = consumed_node.y as isize + y;
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx >= field.len() || ny >= field.len() {
                continue;
            }
            if field[nx][ny] == MapPixel::Wall {
                continue;
            }

            if closed_list
                .iter()
                .any(|(x, y, cost)| *x == nx && *y == ny && *cost <= new_cost)
            {
                continue;
            }
            if let Some((index, (_, cost))) = open_list
                .iter()
                .enumerate()
                .find(|(_, (node, _))| node.x == nx && node.y == ny)
            {
               if *cost > new_cost {
                   open_list.remove(index);
                } else {
                    continue;
                }
            }

            let node = Node {
                x: nx,
                y: ny,
                from: Some(Box::new(consumed_node.clone())),
            };
            field[nx][ny] = MapPixel::Highlighted;
            open_list.push((node, new_cost));
            dead_end = false;
        }
        if dead_end {
            field[consumed_node.x][consumed_node.y] = MapPixel::DeadEnd;
        }

        print_matrix(
            (player_x, player_y),
            (goal_x, goal_y),
            (consumed_node.x, consumed_node.y),
            &field,
        );
        sleep(Duration::from_millis(250));
    }
}

fn heuristics(from: (usize, usize), to: (usize, usize)) -> usize {
    //0
    //((from.0 as isize - to.0 as isize).pow(2) + (from.1 as isize - to.1 as isize).pow(2)) as usize
    (((from.0 as isize) - (to.0 as isize)).abs() + ((from.1 as isize) - (to.1 as isize)).abs()) as usize
}

fn print_matrix(
    player: (usize, usize),
    goal: (usize, usize),
    source: (usize, usize),
    field: &[[MapPixel; FIELD_SIZE]; FIELD_SIZE],
) {
    for _ in 0..10 {
        println!();
    }

    for x in 0..field.len() {
        for y in 0..field[x].len() {
            let print;
            if player.0 == x && player.1 == y {
                print = "P".blue();
            } else if source.0 == x && source.1 == y {
                print = "*".bright_purple();
            } else if goal.0 == x && goal.1 == y {
                print = "G".green();
            } else {
                print = match field[x][y] {
                    MapPixel::Wall => "#".black(),
                    MapPixel::Highlighted => "*".yellow(),
                    MapPixel::DeadEnd => "*".red(),
                    MapPixel::Path => "*".magenta(),
                    MapPixel::Checked => "*".bright_yellow(),
                    MapPixel::Air => "#".normal(),
                };
            }
            print!("{}  ", print);
        }
        println!();
    }
}
