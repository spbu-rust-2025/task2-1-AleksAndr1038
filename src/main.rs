use std::io;

fn manachers_algorithm(input_str: &str) -> Vec<usize> {
    let mut modified_str = String::new();

    modified_str.push('#');
    for ch in input_str.chars() {
        modified_str.push(ch);
        modified_str.push('#');
    }

    let len_str = modified_str.len();
    let mut palindrome_len = vec![0; len_str];
    let mut center = 0;
    let mut right = 0;

    for i in 0..len_str {
        if i < right {
            let mirror = 2 * center - i;
            palindrome_len[i] = palindrome_len[mirror].min(right - i);
        }

        let mut l_border = i + 1 + palindrome_len[i];
        let mut r_border = i as isize - 1 - palindrome_len[i] as isize;

        while l_border < len_str
            && r_border >= 0
            && modified_str.chars().nth(l_border as usize).unwrap()
                == modified_str.chars().nth(r_border as usize).unwrap()
        {
            palindrome_len[i] += 1;
            l_border += 1;
            r_border -= 1;
        }

        if i + palindrome_len[i] > right {
            center = i;
            right = i + palindrome_len[i];
        }
    }

    palindrome_len
}

fn finding_longest_palindrome(input_str: &str) -> String {
    if input_str.is_empty() {
        return String::new();
    }

    let palindrome_len = manachers_algorithm(input_str);

    let mut center = 0;
    let mut max_lenght = 0;

    for (i, &lenght) in palindrome_len.iter().enumerate() {
        if lenght > max_lenght {
            max_lenght = lenght;
            center = i;
        }
    }

    let start = (center - max_lenght) / 2;

    input_str.chars().skip(start).take(max_lenght).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");

    let result = finding_longest_palindrome(input.trim());
    println!("{}", result);
}
