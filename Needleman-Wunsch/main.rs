mod dataframe;

fn needleman_wunsch(seq1: &str, seq2: &str, match_score: i32, mismatch_score: i32, gap_penalty: i32) -> (i32, String, String) {
    let len1 = seq1.len();
    let len2 = seq2.len();

    // Initialize the scoring matrix
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    // Initialize the traceback matrix to store directions
    let mut traceback = vec![vec![0u8; len2 + 1]; len1 + 1];

    // Initialize the first row and column with gap penalties
    for i in 0..=len1 {
        matrix[i][0] = i as i32 * gap_penalty;
        traceback[i][0] = b'|';
    }
    for j in 0..=len2 {
        matrix[0][j] = j as i32 * gap_penalty;
        traceback[0][j] = b'_';
    }

    // Fill the scoring matrix
    for i in 1..=len1 {
        for j in 1..=len2 {
            let score = if seq1.chars().nth(i - 1) == seq2.chars().nth(j - 1) {
                match_score
            } else {
                mismatch_score
            };

            let diag_score = matrix[i - 1][j - 1] + score;
            let up_score = matrix[i - 1][j] + gap_penalty;
            let left_score = matrix[i][j - 1] + gap_penalty;

            matrix[i][j] = diag_score.max(up_score).max(left_score);

            if matrix[i][j] == diag_score {
                traceback[i][j] = b'\\';
            } else if matrix[i][j] == up_score {
                traceback[i][j] = b'|';
            } else {
                traceback[i][j] = b'_';
            }
        }
    }

    // Traceback to find the aligned sequences
    let mut aligned_seq1 = String::new();
    let mut aligned_seq2 = String::new();
    let (mut i, mut j) = (len1, len2);
    while i > 0 || j > 0 {
        match traceback[i][j] {
            b'\\' => {
                aligned_seq1.push(seq1.chars().nth(i - 1).unwrap());
                aligned_seq2.push(seq2.chars().nth(j - 1).unwrap());
                i -= 1;
                j -= 1;
            }
            b'|' => {
                aligned_seq1.push(seq1.chars().nth(i - 1).unwrap());
                aligned_seq2.push('-');
                i -= 1;
            }
            _ => {
                aligned_seq1.push('-');
                aligned_seq2.push(seq2.chars().nth(j - 1).unwrap());
                j -= 1;
            }
        }
    }

    // Reverse the aligned sequences
    let aligned_seq1 = aligned_seq1.chars().rev().collect::<String>();
    let aligned_seq2 = aligned_seq2.chars().rev().collect::<String>();

    (matrix[len1][len2], aligned_seq1, aligned_seq2)
}

fn main() {
    let seq1 = "GCATGCG";
    let seq2 = "GATTACA";
    let (score, aligned_seq1, aligned_seq2) = needleman_wunsch(seq1, seq2, 1, -1, -1);
    println!("Alignment score: {}", score);
    println!("Aligned sequence 1: {}", aligned_seq1);
    println!("Aligned sequence 2: {}", aligned_seq2);
}
