mod iter {
    use loadless::LoadlessIteratorExt;

    #[test]
    #[ignore]
    fn iter_writes_stdout_expensive() {
        let test = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("Starting a long process inside a for loop 😴...\n");
        for _ in test.iter().loadless() {
            std::thread::sleep(std::time::Duration::from_millis(5000));
        }
    }

    #[test]
    fn iter_with_default_works() {
        let test = vec!['a', 'b', 'c', 'd', 'e'];
        let mut target = Vec::new();
        for _ in test.iter().loadless().write_target(&mut target) {}
        assert_eq!(
            String::from_utf8_lossy(&target),
            "[     ]\n\u{1b}[1F\u{1b}[2K[▓    ]\n\u{1b}[1F\u{1b}[2K[▓▓   ]\n\u{1b}[1F\u{1b}[2K[▓▓▓  ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓ ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓]\n"
        );
    }

    #[test]
    fn iter_with_default_above_10_works() {
        let test = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let mut target = Vec::new();
        for _ in test.iter().loadless().write_target(&mut target) {}
        assert_eq!(String::from_utf8_lossy(&target), "[          ]\n\u{1b}[1F\u{1b}[2K[          ]\n\u{1b}[1F\u{1b}[2K[▓         ]\n\u{1b}[1F\u{1b}[2K[▓         ]\n\u{1b}[1F\u{1b}[2K[▓▓        ]\n\u{1b}[1F\u{1b}[2K[▓▓        ]\n\u{1b}[1F\u{1b}[2K[▓▓▓       ]\n\u{1b}[1F\u{1b}[2K[▓▓▓       ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓      ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓      ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓     ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓     ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓    ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓    ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓   ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓   ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓  ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓  ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓▓ ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓▓ ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓▓▓]\n".to_string());
    }

    #[test]
    fn iter_with_default_above_10_uneven_works() {
        let test = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let mut target = Vec::new();
        for _ in test.iter().loadless().write_target(&mut target) {}
        assert_eq!(String::from_utf8_lossy(&target), "[          ]\n\u{1b}[1F\u{1b}[2K[▓         ]\n\u{1b}[1F\u{1b}[2K[▓▓        ]\n\u{1b}[1F\u{1b}[2K[▓▓▓       ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓      ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓     ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓    ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓   ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓  ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓▓ ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓▓▓]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓▓▓▓]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓▓▓▓▓▓▓▓▓]\n".to_string());
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
            "[    ]\n\u{1b}[1F\u{1b}[2K[▓   ]\n\u{1b}[1F\u{1b}[2K[▓▓  ]\n\u{1b}[1F\u{1b}[2K[▓▓▓ ]\n\u{1b}[1F\u{1b}[2K[▓▓▓▓]\n".to_string()
        );
        assert_eq!(test, vec![11, 11, 11, 11]);
    }
}
