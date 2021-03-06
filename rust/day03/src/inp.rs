pub fn input() -> &'static str {
    r#"

.#...#.......#...#...#.#.#.....
####.....#.#..#...#...........#
.....#...........#......#....#.
......#..#......#.#..#...##.#.#
............#......#...........
...........#.#.#....#.......##.
....#.......#..............#...
........##...#.#.....##...##.#.
.#.#.....##................##..
.##................##..#...##..
....#...###...##.........#....#
.##......#.........#...........
...#.#.#....#....#...#...##...#
..#....##...#..#.#..#.....#.#..
.......#...#..#..#.....#...#..#
.....#......#.......#.....#.#..
....#..#...#..#####....##......
.#...........#......#....#....#
#......#.###.....#....#....#...
....#..#.#.#..#...........##...
..#..#..#.#...#......#....#.##.
.##....#......#...#.#..#.......
..###.#...#.........#.#.#...#.#
#....###.........#...#...#...#.
...##.#............#...##......
...#.........#............#....
......##...#...##..#...........
........##..#.#.####...#.....#.
.##.........#......#..#..#...#.
..........#...#..........#.....
#..........#........#..#..#.#..
..#....#.#.#.#.#..#.##.........
##.#.#.##.....#..#......###....
##....#...#.....#..............
.#..#...#...#....###......#....
#....#......#.#..#.#........###
.#....#..#...###....#...##.....
.#....#.....#.....#..##..#.....
#....#.##...#...#..#.##.##.#...
.#.#.#.##...#####.............#
......##..#.....##..#...####...
#.##..#.#....#..##.......###..#
..#.......##....#........#.##..
#.....#......#.....#....#..#...
.......##...#.....##.......#..#
.......#...#.#.#.........#####.
#.......#.##..##........##.....
##..#...#........##....#.......
.......#...##......##...##.##..
......#..##..#.#...#...#....##.
....#.#..#.....#.##.#.....#.#..
#..#.#.#........#...#.......##.
##...........#..#........#.....
....##....#....#.#.......#.....
....##.#.#.....#.#.....#.....#.
..........#.#..##..#..#.......#
#....#.......##...#...#.....#..
.........##.....#.#....#......#
..........#........#..#..#.#...
..#......#.....#......#......#.
..#...###..##..#.....##..#..##.
..#.#..###.........#.#...##.#.#
#.........#..#......#...##.....
...#...#.#..#...##.#...##.#..#.
#.....#.....#.##....#.#......#.
#....##..##..#.#..##....#.....#
.#..........#..#...#..#.......#
#.#.....#..##..##..#.#.........
....#..##...#.....#.....#.#.#.#
...#.#....#........#...#.#.....
.#............#.......#.##.#...
..##.......#.#...#........##..#
..................##.#...#.#..#
.#.........#.......#.....#.....
....##...##..#..........#......
..#.##..#....#..#............#.
....####...#.##....##.#....#.##
#..#....#......##........##....
..###...........##..#......#...
#..#.......#........#.......#..
.....#....#..#..##.....#.......
.###.####.#....#....#..#.......
.............#...............#.
.#..........#.#....#..#.#......
..............##....#..#....##.
.......#.#..#........#.......##
#..#...#..#.#........#..#....#.
...#.........#...#..#..........
...#....##...#..#..........#...
.#......#......##..##...#.#....
.#.........#..###..............
.................#.#.....##....
...#............#..............
#..#................#.......#..
...#.......#......#.#.#........
#.....#.##....#.....#........#.
......##.#....#........#....#..
.#..#.##...##........#.#.....#.
..#...#....#...#..#..##..#.#..#
#.................#.#.......##.
..........#........#.#.....#..#
#....##....#........##..##.#...
#...#....#.....#.....#.....#...
#..#..........#....##....#....#
..#.#..#..#....#.#....#....#..#
#....#..#.......#..........#...
.#...#.#...#..#...#.......#....
###........#......##..#...##...
...#..........##..............#
.......#........##......#.....#
.#..........#...#...##....###.#
.#...#....#..#.....##...#..##..
.#.#.#...##..........##...#...#
.#.....#...#........#........##
#.......#......##.#.#..#....#..
##..#.##........#....#..#......
...#.......................#...
..#....#..##........##.#.##.#..
.............#.......#....#.#.#
...#...........##..#.....#.....
..#....##....#.....#...........
..#.....#......#..#.###.##....#
.#.......#...........#...#....#
#............##...#...#.....#..
##...#.....#.........##...##...
...#...........#....##.........
#.##..#..........##..#......#..
.......#.#.......##.......#....
..#.....##..#...#.......#......
.#........#....##...........#..
#.......#...#.#.###...#....#...
..........##..#..#..##........#
#....#....#...#....#....#......
...........#....#...#...##.#...
.........#.#.....#.............
..####...........##..........#.
.....#...................#.....
#..##...#........#.###.#.##....
....##...#.##................#.
.#........###.#............#.#.
..............#.##.........#...
##............#.#..###....#...#
#.....#........####....#....##.
....##..#...##..##...##.....#..
##..#....#.##.....####.....#.##
##..#....#.##.##.#.#........#..
....#..........##.....#..#..#..
...#.......#........#.........#
#..##.######.......##........#.
###...#...####.......#.....#...
#......#..#.....#......#.....#.
..................##...#.......
....#.#....#......#...#.....##.
..#..#..#..#..#....#.#...#....#
......#....###.................
#.##......#...#......#.........
#..#.#...##..#.......#..#...#..
.#....#.#........#.........#...
#.......##..#..#...............
........#..##....#.....#..#....
....#......##..#....#...#..#...
#.....#...##..#...#......#.....
.....#.....#.........##...#..#.
........#...##.#...#.#....#..##
....#....#...#.....##..#...#...
#....#..#.........#.........###
..###.....##...#.#....##......#
#..#.#..#.......#..#....##.....
###...#.##..#.......#......#...
.....#.....##.......#...##..#..
......#.......#.#.#......#..#..
.................##..#.###.....
..........#....#...#..........#
...#.#...#.#..##.....#.#.##..#.
.......#..#....#...#......###..
...##..........#..#.....#....#.
.#..##..###...#....##.....#....
..#.#..............#....#...#..
.....####.......#.#.##....#....
#.#.#..##.##.#..#.##.#....#..#.
........#....#.......##........
...#...#....#...###.....###....
.....#..#..........##.#...##.##
..#.#.#..#....#...#..##...#...#
..#......#..#.#.....#....#....#
.#.....#.......#............#..
#..##....#...#....##....#......
#..#.........#...#...###.#..#..
..#.#.#..#.#..#.......##.......
...##...............#..#...#.#.
.......####.#.....#..#..#......
......#..#.....#..##....#......
....#...#.........##.......#.#.
#.#.#...#.....#...#..#.#..#....
........#..#.........#..#..##..
........###.#............#.#...
#..#.......#.#..#.......#...#.#
..##..............#.#.....#...#
..##...........................
..#.....#.......#......##......
#...#......##.#....#.#.#...##.#
#...#.#......#.#..##.........#.
.##..#...#.#.....#.#.#...#.#..#
.#..#...#.#.........#......#...
...........#...#...#...#..#.#..
.#........#...#......##...#.###
#........#..#.#..#...........##
.#...#...####.......#..........
......#...............#........
.....#.#.....#.#...#......#....
.#........#...........#..##.#..
....#..#.....###.......#...#...
#.#.........#...##..#.#.##.#...
................##.#....#.#...#
.......#.......#......#...#....
#....#.#..............#.##..###
..##.##..#.....#............#..
#....#..##........#....#.......
.#.#........#.#................
......##..#..#..........#..#.#.
.....##.#..#....##.#......##...
........###.#................#.
#..###.....#.###.#...#.#.......
.#..#.#.#.#..#..#.#.....#.#....
#....#.....#..#......##...#..##
........#...##..#.#.....#....#.
.......#..#..#..#....#.....##..
....#..##..#...#....#.........#
#.#....#..#.#...#.#...#....#...
.....#......###.......#..##.#.#
.......##.....#....#........#.#
.##.##..#..###.#....#.#.....#..
..##.#.......###.........#.....
.#...#......#..#....#..........
.....#........#.....##...#.....
..#......#.#.#..#.#....##.#...#
#.#...#...........##......#....
.................##.....#.#.##.
###..#....#..................#.
##..#.#.#...#....###.#.#...##.#
#.#.#..#....#..............#...
.....#....#......#..#.##.......
#...#...#..###.......#.......#.
.....#.#........#..#...#.#.....
.....#..........#.###.......#..
...#.##.....#....###.....#.....
####........#....#..#.#.##.#...
#......#...##.....#.#..##.#.#.#
.....##....#..#.........##.....
..##....##................##..#
#.....#...##...##.#.....#...#..
..#..#.#.#....#.#.......#......
##.....##......#...#.........#.
#..........#........#.#......#.
.#..#.......#.#.....#..........
.........#.#.......#.#..#..#..#
#......#....#....#..##..##...##
.....#..#...#.......#.....##...
..#.##........#.###...#...#...#
..#..#...........#..........#..
.#.#.#...#.##.#..............#.
....#..##.......#.....#..##..#.
.#.##.#....##........#...##.##.
...#.#...#....#....#......#####
.....#.....##...........#......
#........#.##.......#.#.......#
#...#.......##.#.......#..#.#..
#...##..#....#............#.#..
........#.#..#...#..#...##..##.
#...#....#............#........
#.#.#.#.#....##.....##.........
......##.........#.......#.#..#
...#.#....#........#...........
...#.#.......#.....#...........
##....####......##.##..#.......
#......#...#..#.#..#......#..#.
#......#.#....#....#..#........
..#.###...#.....#........#.#...
..#.....##.....###....#.....#..
#.##.#.....##....#...###.......
###.#....###.#..##.#.......##.#
#..#..##...#.#..........##.##..
.......####.#..#.....##..###...
#...#...##..#..##.......###....
#....#.........##..#.........#.
.....#.#..........#..#...#.#..#
..........#......##..#..#.#....
.#...#...#...#........###....##
#....#.##..........#.#.....#.#.
#....##.#.##..#.......#.#.....#
.##..##..#.#...#.#...........#.
....##..#...#.#.##.#.#...#.....
.#...#.##........#.##..#.#....#
.#.....##.........#.....#......
..#.....#.#..#.##.............#
##....##...#....##........#....
.#....#........#.#..#..#..#.##.
.#........#............#.......
.#..##..##..#..#..####....#....
..#.###....#..#.##......#.#...#
.###..#.#...##....##....#..##.#
....##........#....#.#.#...##..
...#..#....#.#....#...#.#.....#
...##....##..#....#.........#..
.....#..##.###..#.....####.....
...#..#.........#....#.#.##..#.
...#..#...............#..#....#
...........#.....#...####..##.#
..#......#...#....#..#...##.#..
.....#..#...........#.......#.#
##....###...#.........#....#...
...#..##.......#.#.....##....#.
#.#...#.#....#.....#...##.....#
.#...##....#.....#..##.......#.
...#........##..........#.....#
#...##..#.#....###...#..#......
............#.......#......#.#.
......#....#.#...#...#..#......
.#..#......#....#.......#....##
...#...#.......###..###...#....
.............#.#...#..###.....#
.#.....#........#...##....#..#.
.....#.......#######.#.#...#...
    "#
}
