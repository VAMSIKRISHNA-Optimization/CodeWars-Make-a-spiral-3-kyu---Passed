# CodeWars-Make-a-spiral-3-kyu---Passed
Your task, is to create a NxN spiral with a given size.

For example, spiral with size 5 should look like this:

00000
....0
000.0
0...0
00000
and with the size 10:

0000000000
.........0
00000000.0
0......0.0
0.0000.0.0
0.0..0.0.0
0.0....0.0
0.000000.0
0........0
0000000000
Return value should contain array of arrays, of 0 and 1, with the first row being composed of 1s. For example for given size 5 result should be:

[[1,1,1,1,1],[0,0,0,0,1],[1,1,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
Because of the edge-cases for tiny spirals, the size will be at least 5.

General rule-of-a-thumb is, that the snake made with '1' cannot touch to itself.


TEST CASES
#[cfg(test)]
mod tests {
    use super::*;

    fn expected_spiralize(n: usize) -> Vec<Vec<i8>> {
        (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        let min = i.min(j).min(n - i - 1).min(n - j - 1);
                        (if n % 2 == 0 && i == n / 2 && j == n / 2 - 1 {
                            0
                        } else if j == min && i == min + 1 {
                            min % 2
                        } else {
                            1 - min % 2
                        }) as i8
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_5_through_50() {
        for i in 5..=10 {
            assert_eq!(spiralize(i), expected_spiralize(i), "spiralize({})", i);
        }
        for i in 11..=50 {
            assert!(spiralize(i) == expected_spiralize(i), "spiralize({})", i);
        }
    }
}
