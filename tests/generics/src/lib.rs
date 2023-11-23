struct One<A>(A);
struct Test<A, B> {
    a: A,
    b: B
}
impl<A, B> Test<A, B> {
    fn superb() {
        let var_c = 3;
    }
}
