mod iter {
    use loadless::LoadlessIteratorExt;

    #[test]
    #[ignore]
    fn iter_writes_stdout_expensive() {
        let test = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        for _ in test.iter().loadless() {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    #[test]
    fn iter_with_default_works() {
        let test = vec!['a', 'b', 'c', 'd', 'e'];
        let mut target = Vec::new();
        for _ in test.iter().loadless().write_target(&mut target) {}
        assert_eq!(
            String::from_utf8_lossy(&target),
            "\r[     ]\r[▓    ]\r[▓▓   ]\r[▓▓▓  ]\r[▓▓▓▓ ]\r[▓▓▓▓▓]\n"
        );
    }

    #[test]
    fn iter_mut_with_default_works() {
        let mut test = vec![10, 10, 10, 10];
        let mut target = Vec::new();
        for i in test.iter_mut().loadless().write_target(&mut target) {
            *i += 1;
        }
        assert_eq!(
            String::from_utf8_lossy(&target),
            "\r[    ]\r[▓   ]\r[▓▓  ]\r[▓▓▓ ]\r[▓▓▓▓]\n".to_string()
        );
        assert_eq!(test, vec![11, 11, 11, 11]);
    }
}
