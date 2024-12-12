fn main() {
    let mut stones = INPUT.to_vec();
    for i in 0..75 {
        println!("step {} (len={})", i, stones.len());
        let mut new_stones = Vec::new();
        for s in stones.iter() {
            if *s == 0 {
                new_stones.push(1);
                continue
            }
            let ds = s.ilog10() + 1;
            if ds % 2 == 0 {
                let a = s/(10_usize.pow(ds));
                new_stones.push(a);
                new_stones.push(s-a);
            } else {
                new_stones.push(s*2024);
            }
        }
        stones = new_stones;
    }
    dbg!(stones.len());
}

const INPUT: [usize; 8] = [3935565, 31753, 437818, 7697, 5, 38, 0, 123];