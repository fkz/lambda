use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, 
    BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use criterion_cycles_per_byte::CyclesPerByte;
use lambda_calculus::{execute, parse_arguments, pretty, Program};
use pprof::criterion::{Output, PProfProfiler};

fn do_benchmark<W: Measurement>(c: &mut BenchmarkGroup<'_, W>, id: BenchmarkId, program: Program) {

    let mut steps = 0;
    execute(program.clone(), false, false, false, true, true, &mut steps);
    c.throughput(Throughput::Elements(steps));

    c.bench_with_input(id, &program, |b, p| {
        let mut step_count = 0;
        b.iter(|| execute(black_box(p.clone()), false, false, false, true, true, &mut step_count))
    });
}

fn benchmark_function<W: Measurement>(c: &mut BenchmarkGroup<'_, W>, path: &str, arguments: Vec<u32>) {
    let id = BenchmarkId::from_parameter(format!("{:5}, {:5}", arguments[0], arguments[1]));

    let program = parse_arguments(path, &pretty::number_u32(), arguments);

    do_benchmark(c, id, program);
}

fn b<W: Measurement>(c: &mut Criterion<W>) {
    let values = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    let mut group = c.benchmark_group("small-add");

    for i in values.iter() {
        group.throughput(Throughput::Elements(2 * *i as u64));
        benchmark_function(&mut group, "examples/add", vec![*i, *i]);
    }
    group.finish();

    let mut group = c.benchmark_group("small-sub");
    for i in values.iter().take(5) {
        group.throughput(Throughput::Elements(3 * *i as u64));
        benchmark_function(&mut group, "examples/sub", vec![*i * 2, *i]);
    }
    group.finish();

    let mut group = c.benchmark_group("big-add");
    group.sample_size(10);
    for i in values.iter().take(3) {
        group.throughput(Throughput::Elements(2 * *i as u64));
        benchmark_function(&mut group, "examples/add", vec![*i * 1000, *i * 1000]);
    }
    group.finish();

    let mut group = c.benchmark_group("big-sub");
    group.sample_size(10);
    for i in values.iter().take(3) {
        group.throughput(Throughput::Elements(3 * *i as u64));
        benchmark_function(&mut group, "examples/sub", vec![*i * 4, *i * 2]);
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = b
}
criterion_main!(benches);
