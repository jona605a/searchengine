#![allow(non_snake_case)]
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs,
};

use super::{index10_1::boyer_moore_preprocess, Index};

pub fn apostolico_giancarlo(
    p: &Vec<char>,
    t: &Vec<char>,
    (L_prime, l_prime, R, N): (
        &Vec<usize>,
        &Vec<usize>,
        &HashMap<&char, Vec<usize>>,
        &Vec<usize>,
    ),
) -> Vec<usize> {
    // Search stage
    let n = p.len();
    let m = t.len();
    let mut j = n - 1;
    let mut M: Vec<i32> = vec![-1; m];
    let mut match_indices: Vec<usize> = Vec::new();

    // dbg!(&N);
    while j < m {
        let mut i = n - 1;
        let mut h = j;

        'phase: loop {
            // dbg!((i, h, j), M[h]);
            if M[h] <= 0 {
                if i == 0 && p[i] == t[h] {
                    // P matches T!
                    match_indices.push(j);
                    j += n - l_prime[1];
                    break 'phase;
                } else if p[i] == t[h] {
                    // Match as long as we can
                    i -= 1;
                    h -= 1;
                    continue 'phase;
                } else {
                    // p[i] != t[h]
                    M[j] = (j - h) as i32;
                    j += calculate_shift_at_mismatch(&t[h], i, n, (&L_prime, &l_prime, &R));
                    break 'phase;
                }
            }
            if (M[h] as usize) < N[i] || (M[h] as usize == N[i] && N[i] < i + 1) {
                // Rule 2 and 5
                i -= M[h] as usize;
                h -= M[h] as usize;
                continue 'phase;
            }
            if M[h] as usize >= N[i] && N[i] == i + 1 {
                M[j] = (j - h) as i32;
                // Occurence at position j
                match_indices.push(j);
                j += n - l_prime[1];
                break 'phase;
            }
            if M[h] as usize > N[i] && N[i] < i + 1 {
                assert_ne!(p[i - N[i]], t[h - N[i]]);
                M[j] = (j - h) as i32;
                j += calculate_shift_at_mismatch(
                    &t[h - N[i]],
                    i - N[i],
                    n,
                    (&L_prime, &l_prime, &R),
                );
                break 'phase;
            }
            panic!("This case should be unreachable as N[i] is always <= i + 1")
        }
    }
    match_indices
}

pub fn apostolico_giancarlo_truefalse(
    p: &Vec<char>,
    t: &Vec<char>,
    (L_prime, l_prime, R, N): (
        &Vec<usize>,
        &Vec<usize>,
        &HashMap<&char, Vec<usize>>,
        &Vec<usize>,
    ),
) -> bool {
    // Search stage
    let n = p.len();
    let m = t.len();
    let mut j = n - 1;
    let mut M: Vec<i32> = vec![-1; m];
    while j < m {
        let mut i = n - 1;
        let mut h = j;

        'phase: loop {
            // dbg!((i, h, j), M[h]);
            if M[h] <= 0 {
                if i == 0 && p[i] == t[h] {
                    // P matches T!
                    return true;
                } else if p[i] == t[h] {
                    // Match as long as we can
                    i -= 1;
                    h -= 1;
                    continue 'phase;
                } else {
                    // p[i] != t[h]
                    M[j] = (j - h) as i32;
                    j += calculate_shift_at_mismatch(&t[h], i, n, (&L_prime, &l_prime, &R));
                    break 'phase;
                }
            }
            if (M[h] as usize) < N[i] || (M[h] as usize == N[i] && N[i] < i + 1) {
                // Rule 2 and 5
                i -= M[h] as usize;
                h -= M[h] as usize;
                continue 'phase;
            }
            if M[h] as usize >= N[i] && N[i] == i + 1 {
                M[j] = (j - h) as i32;
                // Occurence at position j
                return true;
            }
            if M[h] as usize > N[i] && N[i] < i + 1 {
                assert_ne!(p[i - N[i]], t[h - N[i]]);
                M[j] = (j - h) as i32;
                j += calculate_shift_at_mismatch(
                    &t[h - N[i]],
                    i - N[i],
                    n,
                    (&L_prime, &l_prime, &R),
                );
                break 'phase;
            }
            panic!("This case should be unreachable as N[i] is always <= i + 1")
        }
    }
    false
}

fn calculate_shift_at_mismatch(
    th: &char,
    i: usize,
    n: usize,
    (L_prime, l_prime, R): (&Vec<usize>, &Vec<usize>, &HashMap<&char, Vec<usize>>),
) -> usize {
    // Bad character rule
    let bc_shift = match R.get(th) {
        Some(c_pos) => {
            let mut temp: i32 = -1;
            for ch_i in c_pos {
                if *ch_i < i {
                    temp = *ch_i as i32;
                    break;
                }
            }
            ((i as i32) - (temp)) as usize
        }
        None => i + 1,
    };
    // Good suffix rule
    let gs_shift = if i >= n - 1 {
        1
    } else {
        if L_prime[i + 1] != 0 {
            n - L_prime[i + 1]
        } else {
            n - l_prime[i + 1]
        }
    };
    // dbg!(i, k, bc_shift, gs_shift);
    max(bc_shift, gs_shift)
}

impl Index<HashMap<String, HashSet<usize>>> {
    pub fn apostolico_giancarlo_search(&self, query: &String) -> Vec<String> {
        let p: Vec<char> = query.chars().collect();

        // Split sentence into words
        // Get article set for each word, and find intersection
        let mut x = query
            .split(' ')
            .map(|w| self.database.get(w).unwrap_or(&HashSet::new()).to_owned());
        let keys = x.next().unwrap();
        let art_intersect: Vec<usize> = keys
            .into_iter()
            .filter(|ar_no| x.all(|hs_a| hs_a.contains(ar_no)))
            .collect();

        let mut result: Vec<usize> = Vec::new();
        let (L_prime, l_prime, R, N) = boyer_moore_preprocess(&p);

        for art_no in art_intersect {
            // Read the file
            let t: Vec<char> =
                fs::read_to_string(format!("data/individual_articles/{:08}.txt", art_no))
                    .expect(
                        format!(
                            "Article number {} not found in data/individual_articles/",
                            art_no
                        )
                        .as_str(),
                    )
                    .chars()
                    .collect();
            if apostolico_giancarlo_truefalse(&p, &t, (&L_prime, &l_prime, &R, &N)) {
                result.push(art_no) // There was at least one occurence
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

    #[test]
    fn apostolico_giancarlo_let_it_be() {
        let t: Vec<char> =
            "wisdom, let it be And words of wisdom, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be And when the broken hearted people living in the world agree There will be an answer, let it be For though they may be parted, there is still a chance that they will see There will be an answer, let it be Let it be let it be let it be let it be There will be an answer, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be be And when the night is cloudy there is still a light that shines on me Shinin' until tomorrow, let it be I wake up to the sound of music, Mother Mary comes to me Speaking words of wisdom, let it be And let it be let it be let it be let it be Whisper words of wisdom, let it be And let it be let it be let it be let it be Whisper words of wisdom, let it be"
                .to_string()
                .to_ascii_lowercase().chars().collect();
        let p: Vec<char> = "let it be".chars().collect();

        let (L_prime, l_prime, R, N) = boyer_moore_preprocess(&p);
        assert_eq!(
            apostolico_giancarlo(&p, &t, (&L_prime, &l_prime, &R, &N)),
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

    #[test]
    fn apostolico_giancarlo_abcab() {
        let t: Vec<char> = "cbacbacbcababababbcbcbcbcaaacbcbcbababcbcbacbabcabcab cba bc abc bacbabcabc bac babcabcbacbabcabcbacbababcabcbacbacbbac"
            .to_string()
            .to_ascii_lowercase()
            .chars()
            .collect();
        let p: Vec<char> = "abcab".chars().collect();
        let (L_prime, l_prime, R, N) = boyer_moore_preprocess(&p);
        assert_eq!(
            apostolico_giancarlo(&p, &t, (&L_prime, &l_prime, &R, &N)),
            vec![49, 52, 73, 85, 95, 107]
        );
    }
}
