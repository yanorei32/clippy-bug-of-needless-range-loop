fn main() {
    let frames = [["a", "b", "c", "d"], ["A", "B", "C", "D"]];
    let channel_count = 2;
    let frame_count = 4;

    for j in 0..frame_count {
        for i in 0..channel_count {
            print!("{}", frames[i][j]);
        }
    }

    println!();
}
