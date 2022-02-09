use csv_parser::*;

#[test]
fn parse_csv() {
    let csv = b"test,\"this\"\"ffff\",\"crazy\nnewline\"\n\"actual next line\"\"\",0".to_vec();
    let check = parse(csv).unwrap();

    let result = vec![
        vec![
            "test".to_string(),
            "this\"ffff".to_string(),
            "crazy\nnewline".to_string(),
        ],
        vec!["actual next line\"".to_string(), "0".to_string()],
    ];

    assert_eq!(check, result);
}

#[test]
fn row_cols_to_lines() {
    let row_cols = vec![
        vec!["A1".to_string(), "A2".to_string(), "a3".to_string()],
        vec!["b1".to_string(), "B2".to_string(), "B3".to_string()],
        vec!["c1".to_string(), "C2".to_string(), "c3".to_string()],
    ];

    let conversion = to_lines(row_cols);
    let result = vec![
        "| A1 | A2 | a3 | ".to_string(),
        "| b1 | B2 | B3 | ".to_string(),
        "| c1 | C2 | c3 | ".to_string(),
    ];
    //        println!("Conversion:");
    //        println!("{:?}", conversion);
    //        println!();
    //        println!("Expected Result:");
    //        println!("{:?}", result);

    assert_eq!(conversion, result);
}
