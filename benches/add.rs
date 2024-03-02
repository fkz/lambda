use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, profiler::Profiler, BenchmarkGroup, BenchmarkId, Criterion, Throughput
};
use lambda_calculus::{execute, parse_arguments, pretty, Program};
use pprof::criterion::{Output, PProfProfiler};

fn do_benchmark(c: &mut BenchmarkGroup<'_, WallTime>, id: BenchmarkId, program: Program) {
    c.bench_with_input(id, &program, |b, p| {
        b.iter(|| execute(black_box(p.clone()), false, false, false, true))
    });
}

fn benchmark_function(c: &mut BenchmarkGroup<'_, WallTime>, path: &str, arguments: Vec<u32>) {
    let id = BenchmarkId::from_parameter(format!("{}, {}", arguments[0], arguments[1]));

    let program = parse_arguments(path, &pretty::number_u32(), arguments);

    do_benchmark(c, id, program);
}

fn b(c: &mut Criterion) {
    let values = [
        0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384,
    ];

    let mut group = c.benchmark_group("add");
    for i in values.iter() {
        group.throughput(Throughput::Elements(2 * *i as u64));
        benchmark_function(&mut group,"examples/add", vec![*i, *i]);
    }
    group.finish();

    let mut group = c.benchmark_group("sub");
    for i in values.iter().take(7) {
        group.throughput(Throughput::Elements(3 * *i as u64));
        benchmark_function(&mut group,  "examples/sub", vec![*i * 2, *i]);
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = b
}
criterion_main!(benches);
