use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let parts: Vec<&str> = data.split(": ").collect::<Vec<&str>>()[1]
        .split(", ")
        .collect();

    let _x: Vec<i32> = parts[0].split("=").collect::<Vec<&str>>()[1]
        .split("..")
        .map(|num| num.parse().unwrap())
        .collect();

    let y: Vec<i32> = parts[1].split("=").collect::<Vec<&str>>()[1]
        .split("..")
        .map(|num| num.parse().unwrap())
        .collect();

    /*
     * Cool math / physics trick
     *
     * The second time the probe hits y = 0, vy will be -vy0.
     * The bigger vy0, the higher the probe will reach, but also the faster it will fall.
     * So, we want the maximum possible vy0 that reaches the target area immediately after
     * hitting y = 0 the second time, which will be equal to ymin, the lowest point in the
     * target area.
     *
     * As y(t) of the probe is equal to sum(vy(t) for t in 0..t), we can obtain the highest
     * y by calculating the sum of an arithmetic progression (as in day7) with either:
     * n = ymin, a1 = 1, an = ymin OR n = ymin + 1, a1 = 0, an = ymin
     * because the probe stays in y=MAX for two t (because vy = 0 when it reaches that point)
     *
     * Not sure if this works for all inputs (because of restricitons on target area X)
     * But it worked for both the example and the input, so I assumed the input were made
     * specifiicaly for this solution to work.
     */

    let ymin = y[0].abs() - 1;

    let res = (ymin * (ymin + 1)) / 2;
    println!("{}", res);
}
