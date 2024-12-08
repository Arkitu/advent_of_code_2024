fn main() {
    let mut lines = INPUT.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let X = lines[0].len();
    let Y = lines.len();
    let mut antenas = Vec::new();
    for y in 0..Y {
        for x in 0..Y {
            if lines[y][x] != '.' {
                antenas.push((x, y, lines[y][x]));
            }
        }
    }
    for a in antenas.iter() {
        for b in antenas.iter() {
            if (a.0 == b.0 && a.1 == b.1) || a.2 != b.2 {
                continue
            }
            let x: isize = (2*(isize::try_from(b.0).unwrap())) - (isize::try_from(a.0).unwrap());
            let y: isize = (2*(isize::try_from(b.1).unwrap())) - (isize::try_from(a.1).unwrap());
            if x >= isize::try_from(X).unwrap() || y >= isize::try_from(Y).unwrap() || x < 0 || y < 0 {
                continue
            }
            lines[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] = '#';
        }
    }
    let count: usize = lines.iter().fold(0, |acc, l| acc+l.iter().fold(0, |acc, c| if *c == '#' {acc+1} else {acc}));
    dbg!(count);
}

const TEST_INPUT: &'static str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

const INPUT: &'static str = ".............O.....E...M.................S........
....................................S.........D...
......u......................................V....
................u.E......M..............V.........
...e...............................a..............
.................v..........a6....................
...............ue...............6...............V.
..........e..E..........M......................V..
............................M.............F.a.....
.............................a..........i.........
....................n.............A......F........
.......e............N.........6...........m.......
........v..............D.....i....................
........A.v......D...........................F....
...................v..........S....q.......m......
................n.......2......m..................
.I..................2...C.....i...............f...
.....................2.S.........i................
........k...0.....................................
......I..........k.......................1........
..............D....k..N....1......................
.....A.................n..C.......................
...........................2..1........fm....A....
.....I........k...7............................f..
....I..........n.........N...d...C................
.............z...................W................
...Z.................................W.........y..
............z....w.....................5..........
.................z....d...4......Q.U......5.......
....................d...........F.......fC.U......
...................w........................U.....
.............8......w..N..................U.......
.7.................4...3.............5............
...............3.......s..1.54..........W.........
.........0Z.....w.............................c...
.Z...................................o............
................z....8...........K......Q.9....c..
............P.........4dY...s9...........O..p....u
.....................s..........o...............W.
....................3.......................O.....
........................s..............p......O...
...................c.............P..9..K.....p....
................Y....P3.....7..9..................
..................8.......o...........K...c.......
.......Y........8.................................
....Z7.....................y.q......Q.............
..............................y...................
......0.....................y..................Q..
....................P6............qo......p.......
......................................q...........";