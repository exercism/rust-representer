fn main() {
    let x = 5;

    let z = {
        let y = 6;
        x + y
    };

    x + z
}