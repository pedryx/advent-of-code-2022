struct Tile {
    height: u32,
    visible: bool,
    score: u32,
}

fn main() {
    let mut size = 0;
    let mut data = include_str!("../in.txt")
        .lines()
        .map(|l| {
            size += 1;
            l.chars().map(|c| Tile {
                height: c.to_digit(10).unwrap(),
                visible: false,
                score: 1
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    for i in 0..size {
        let mut max = vec![
            (i, 0),           // left to right
            (i, size - 1),    // right to right
            (0, i),           // top to bottom
            (size - 1, i),    // bottom to top
        ];
        max.iter().for_each(|pos| data[pos.0][pos.1].visible = true);

        for j in 0..size {
            let current = vec![
                (i, j),               // left to right
                (i, size - j - 1),    // right to left
                (j, i),               // top to bottom
                (size - j - 1, i)     // bottom to top
            ];
            current.iter().zip(max.iter_mut()).for_each(|(pos, max)| {
                if data[pos.0][pos.1].height > data[max.0][max.1].height {
                    data[pos.0][pos.1].visible = true;
                    *max = *pos;
                }
            });

            let current_height = data[i][j].height;
            // view left
            let mut score = 0;
            for x in (0..j).rev() {
                score += 1;
                if data[i][x].height >= current_height {
                    break;
                }
            }
            data[i][j].score *= score;
            // view right
            score = 0;
            for x in j+1..size {
                score += 1;
                if data[i][x].height >= current_height {
                    break;
                }
            }
            data[i][j].score *= score;
            // view up
            score = 0;
            for y in (0..i).rev() {
                score += 1;
                if data[y][j].height >= current_height {
                    break;
                }
            }
            data[i][j].score *= score;
            // view down
            score = 0;
            for y in i+1..size {
                score += 1;
                if data[y][j].height >= current_height {
                    break;
                }
            }
            data[i][j].score *= score;
        }
    }

    let result1: usize = data.iter()
        .map(|row| row.iter().filter(|tile| tile.visible).count())
        .sum();
    let result2 = data.iter()
        .map(|row| row.iter().map(|tile| tile.score).max().unwrap())
        .max().unwrap();

    println!("part1: {}", result1);
    println!("part2: {}", result2);
}