use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lambda_calculus::{execute, parse_arguments, pretty, Program};

fn do_benchmark(c: &mut Criterion, name: &str, program: Program) {
    c.bench_function(name, |b| b.iter(|| execute(black_box(program.clone()), false, false, false, true)));
}

fn benchmark_function(c: &mut Criterion, name: &str, path: &str, arguments: Vec<u32>) {
    let program = parse_arguments(
        path,
        &pretty::number_u32(),
        arguments,
    );
    do_benchmark(c, name, program);
}

fn b(c: &mut Criterion) {
    benchmark_function(c, "add", "examples/add",vec![10000, 10000]);
    benchmark_function(c, "sub", "examples/sub",vec![50, 25]);
}

criterion_group!(benches, b);
criterion_main!(benches);
