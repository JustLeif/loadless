mod iter {
    use loadless::iter::LoadlessIterExt;

    #[test]
    fn iter_writes_stdout() {}
    #[test]
    fn iter_works() {}

    #[test]
    fn iter_mut_works() {
        let mut test = vec![10, 10, 10, 10];
        let mut write_target = Vec::new();
        for i in test.iter_mut().loadless_write_target(&mut write_target) {
            *i += 1;
        }
        println!("{}", String::from_utf8(write_target).unwrap());
        assert_eq!(test, vec![11, 11, 11, 11]);
    }
}
