fn fuzzy_match(needle: &str, haystack: &str) -> bool {
    let mut haystack_chars = haystack.chars();
    'N: for needle_ch in needle.chars() {
        while let Some(haystack_ch) = haystack_chars.next() {
            if needle_ch == haystack_ch { continue 'N; }
        }
        return false;
    }
    true
}

#[derive(Debug)]
struct FuzzyMatch<'a> {
    target: &'a str,
    submatches: Vec<std::ops::Range<usize>>,
}

fn fuzzy_match_anchored<'n, 'h>(needle: &'n str, haystack: &'h str) -> Option<FuzzyMatch<'h>> {
    let mut submatches = Vec::new();
    let mut submatch_start = None;
    let mut haystack_ichars = haystack.char_indices();
    'N: for needle_ch in needle.chars() {
        while let Some((haystack_i, haystack_ch)) = haystack_ichars.next() {
            if needle_ch == haystack_ch {
                if submatch_start == None {
                    submatch_start = Some(haystack_i);
                }
                continue 'N;
            }
            if let Some(start) = submatch_start {
                submatches.push(start .. haystack_i);
                submatch_start = None;
            }
        }
        return None;
    }
    if let Some(start) = submatch_start {
        submatches.push(start .. haystack.len());
    }
    Some(FuzzyMatch { target: haystack, submatches: submatches })
}

fn main() {
    println!("{}", fuzzy_match("ace", "abcdef"));
    println!("{:?}", fuzzy_match_anchored("bcf", "abcdef"));
}
