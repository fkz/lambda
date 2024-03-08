use lambda_calculus::{execute, parse_arguments, pretty};

const FIRST: u32 = 100000;
const SECOND: u32 = 100000;

#[test]
fn add_big_numbers() {
    let program = parse_arguments("examples/add", &pretty::number_u32(), vec![FIRST, SECOND]);
    let mut steps = 0;
    let simplified = execute(
        program, false, false, false, true, true, false, &mut steps, None,
    );
    assert_eq!(
        pretty::number_u32().program_to_string(&simplified),
        Some(FIRST + SECOND)
    );
}
