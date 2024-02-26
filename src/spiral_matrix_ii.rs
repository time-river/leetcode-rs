pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut seen = vec![vec![false; n.try_into().unwrap()]; n.try_into().unwrap()];
    let dr: Vec<i32> = vec![0, 1, 0, -1];
    let dc: Vec<i32> = vec![1, 0, -1, 0];

    let mut r = 0i32;
    let mut c = 0i32;
    let mut di = 0;
    let mut ans = vec![vec![0; n.try_into().unwrap()]; n.try_into().unwrap()];

    for i in 1..(n.pow(2)+1) {
        ans[r as usize][c as usize] = i;
        seen[r as usize][c as usize] = true;

        let nr = r + dr[di];
        let nc = c + dc[di];

        if 0 <= nr && nr < n
                && 0 <= nc && nc < n
                && !seen[nr as usize][nc as usize] {
            r = nr;
            c = nc;
        } else {
            di = (di + 1) % 4;
            r += dr[di];
            c += dc[di];
        }
    }

    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(generate_matrix(3), [[1,2,3],[8,9,4],[7,6,5]]);
    }
}