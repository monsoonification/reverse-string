
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut output_string = String::new();
    let input_cluster = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let mut temp = String::new();

    for i in 0..input_cluster.len() {
        temp = output_string;
        output_string = input_cluster[i].to_string();
        output_string.push_str(&temp);
    }
    return output_string;
}
