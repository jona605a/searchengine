#![allow(non_snake_case)]
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs,
};

use super::Index;

pub fn z_alg(s: &Vec<&char>) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i < r {
            z[i] = min(r - i + 1, z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1
        }
    }
    z
}

fn compute_L_primes(N: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    // Compute L'(i), l'(i)
    let n = N.len();
    let mut L_prime = vec![0; n];
    let mut l_prime = vec![0; n];
    for j in 1..n {
        let i = n - N[j - 1];
        if i < n {
            L_prime[i] = j;
        }

        l_prime[n - j] = max(
            if N[j - 1] == j { N[j - 1] } else { 0 },
            if 1 < j { l_prime[n - j + 1] } else { 0 },
        );
    }
    (L_prime, l_prime)
}

fn compute_R(p: &Vec<char>) -> HashMap<&char, Vec<usize>> {
    let n = p.len();
    let mut R: HashMap<&char, Vec<usize>> = HashMap::new();
    for i in 0..n {
        R.entry(&p[n - 1 - i]).or_default().push(n - 1 - i)
    }
    R
}

pub fn boyer_moore_preprocess(
    p: &Vec<char>,
) -> (Vec<usize>, Vec<usize>, HashMap<&char, Vec<usize>>) {
    // Compute N[j](P) values
    let p_rev: Vec<&char> = p.iter().rev().collect();
    let N: Vec<usize> = z_alg(&p_rev).iter().rev().map(|x| *x).collect();

    // Compute L' values
    let (L_prime, l_prime) = compute_L_primes(N);

    // Compute R values
    let R: HashMap<&char, Vec<usize>> = compute_R(p);

    (L_prime, l_prime, R)
}

pub fn boyer_moore(
    p: &Vec<char>,
    t: &Vec<char>,
    (L_prime, l_prime, R): (&Vec<usize>, &Vec<usize>, &HashMap<&char, Vec<usize>>),
) -> Vec<usize> {
    // Search stage
    let n = p.len();
    let m = t.len();
    let mut k = n - 1;
    let mut match_indices: Vec<usize> = Vec::new();
    while k < m {
        let mut i = n - 1;
        let mut h = k;
        while i > 0 && p[i] == t[h] {
            // Match as long as we can
            i -= 1;
            h -= 1;
        }
        if i == 0 && p[i] == t[h] {
            // P matches T!
            println!("Match!");
            match_indices.push(k);
            k += n - l_prime[1];
        } else {
            // Bad character rule
            let bc_shift = match R.get(&t[h]) {
                Some(c_pos) => {
                    let mut temp: i32 = -1;
                    for ch_i in c_pos {
                        if *ch_i < i {
                            temp = *ch_i as i32;
                            break;
                        }
                    }
                    ((i as i32) - (temp + 1)) as usize
                }
                None => i + 1,
            };

            // Good suffix rule
            let gs_shift = if i == n - 1 {
                1
            } else if i + 1 < n {
                if L_prime[i + 1] != 0 {
                    n - L_prime[i + 1]
                } else {
                    n - l_prime[i + 1]
                }
            } else {
                1
            };

            dbg!(i, k, bc_shift, gs_shift);

            k += max(bc_shift, gs_shift)
        }
    }

    match_indices
}

impl Index<HashMap<String, HashSet<usize>>> {
    pub fn boyer_moore_search(&self, query: &String) -> Vec<String> {
        let p: Vec<char> = query.chars().collect();

        // Split sentence into words
        // Get article set for each word, and find intersection
        let mut x = query
            .split(' ')
            .map(|w| self.database.get(w).unwrap_or(&HashSet::new()).to_owned());
        let keys = x.next().unwrap();
        let art_intersect: Vec<usize> = keys.into_iter().filter(|ar_no| x.all(|hs_a| hs_a.contains(ar_no))).collect();

        // let query_words: Vec<&str> = query.split(' ').collect();
        let mut result: Vec<usize> = Vec::new();
        let (L_prime, l_prime, R) = boyer_moore_preprocess(&p);

        for art_no in art_intersect {
            // Read the file
            let t: Vec<char> =
                fs::read_to_string(format!("data/individual_articles/{:05}.txt", art_no))
                    .expect(format!("Article number {} not found in data/individual_articles/", art_no).as_str(),)
                    .chars()
                    .collect();
            match boyer_moore(&p, &t, (&L_prime, &l_prime, &R)) {
                x if x == vec![] => (),   // Empty vector
                _ => result.push(art_no), // There was at least one occurence
            }
        }
        // Result to article names
        result
            .iter()
            .map(|a_no| self.article_titles[*a_no].to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // #[ignore]
    // fn test_z_alg() {
    //     for _ in 0..100000 {
    //         let n = 50;
    //         let s: String = rand::thread_rng()
    //             .sample_iter(Uniform::new(0, 5))
    //             .take(n)
    //             .map(|i| char::from_digit(i, 10).unwrap())
    //             .collect();
    //         z_alg(&s.chars().collect());
    //     }
    // }

    #[test]
    fn correct_n_values_from_z_alg() {
        let P: Vec<char> = "cabdabdab".chars().collect();
        let p_rev: Vec<&char> = P.iter().rev().collect();
        let N: Vec<usize> = z_alg(&p_rev).iter().rev().map(|x| *x).collect();
        assert_eq!(N, vec![0, 0, 2, 0, 0, 5, 0, 0, 0])
    }

    #[test]
    fn correct_l_primes() {
        let P: Vec<char> = "cabdabdab".chars().collect();
        let p_rev: Vec<&char> = P.iter().rev().collect();
        let N: Vec<usize> = z_alg(&p_rev).iter().rev().map(|x| *x).collect();

        assert_eq!(
            compute_L_primes(N),
            (vec![0, 0, 0, 0, 6, 0, 0, 3, 0], vec![0; 9])
        );

        let P: Vec<char> = "tapFtapGtapFtap".chars().collect();
        let p_rev: Vec<&char> = P.iter().rev().collect();
        let N: Vec<usize> = z_alg(&p_rev).iter().rev().map(|x| *x).collect();

        assert_eq!(
            compute_L_primes(N),
            (
                vec![0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 11, 0, 0],
                vec![0, 7, 7, 7, 7, 7, 7, 7, 7, 3, 3, 3, 3, 0, 0]
            )
        );
    }

    #[test]
    fn correct_boyer_moore_let_it_be() {
        let t: Vec<char> =
            "wisdom, let it be And words of wisdom, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be And when the broken hearted people living in the world agree There will be an answer, let it be For though they may be parted, there is still a chance that they will see There will be an answer, let it be Let it be let it be let it be let it be There will be an answer, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be be And when the night is cloudy there is still a light that shines on me Shinin' until tomorrow, let it be I wake up to the sound of music, Mother Mary comes to me Speaking words of wisdom, let it be And let it be let it be let it be let it be Whisper words of wisdom, let it be And let it be let it be let it be let it be Whisper words of wisdom, let it be"
                .to_string()
                .to_ascii_lowercase().chars().collect();
        let p: Vec<char> = "let it be".chars().collect();

        let p_rev: Vec<&char> = p.iter().rev().collect();
        let N: Vec<usize> = z_alg(&p_rev).iter().rev().map(|x| *x).collect();
        dbg!(compute_L_primes(N));
        let (L_prime, l_prime, R) = boyer_moore_preprocess(&p);
        assert_eq!(
            boyer_moore(&p, &t, (&L_prime, &l_prime, &R)),
            vec![
                17, 48, 58, 68, 78, 88, 123, 219, 328, 338, 348, 358, 368, 403, 413, 423, 433, 443,
                478, 488, 498, 508, 518, 553, 660, 753, 767, 777, 787, 797, 832, 846, 856, 866,
                876, 911
            ]
            .iter()
            .map(|x| x - 1)
            .collect::<Vec<usize>>()
        );
    }

    // "wisdom, let it be And words of wisdom, let it be Let it be let it be let it be let it be Whisper wor
    //  ds of wisdom, let it be And when the broken hearted people living in the world agree There will be a
    //  n answer, let it be For though they may be parted, there is still a chance that they will see There
    //  will be an answer, let it be Let it be let it be let it be let it be There will be an answer, let it
    //   be Let it be let it be let it be let it be Whisper words of wisdom, let it be Let it be let it be l
    //  et it be let it be Whisper words of wisdom, let it be be And when the night is cloudy there is still
    //   a light that shines on me Shinin' until tomorrow, let it be I wake up to the sound of music, Mother
    //   Mary comes to me Speaking words of wisdom, let it be And let it be let it be let it be let it be Wh
    //  isper words of wisdom, let it be And let it be let it be let it be let it be Whisper words of wisdom
    //  , let it be"

    #[test]
    fn correct_boyer_moore_abcab() {
        let t: Vec<char> = "cbacbacbcababababbcbcbcbcaaacbcbcbababcbcbacbabcabcab cba bc abc bacbabcabc bac babcabcbacbabcabcbacbababcabcbacbacbbac"
            .to_string()
            .to_ascii_lowercase()
            .chars()
            .collect();
        let p: Vec<char> = "abcab".chars().collect();
        let (L_prime, l_prime, R) = boyer_moore_preprocess(&p);
        assert_eq!(
            boyer_moore(&p, &t, (&L_prime, &l_prime, &R)),
            vec![49, 52, 73, 85, 95, 107]
        );
    }
}