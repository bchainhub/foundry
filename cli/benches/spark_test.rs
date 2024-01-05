use criterion::{criterion_group, criterion_main, Criterion};
use foxar_cli_test_utils::{util::setup_spark_remote, TestCommand, TestProject};

/// Returns a cloned and `spark built` `solmate` project
fn built_solmate() -> (TestProject, TestCommand) {
    setup_spark_remote("transmissions11/solmate")
}

fn spark_test_benchmark(c: &mut Criterion) {
    let (prj, _) = built_solmate();

    let mut group = c.benchmark_group("spark test");
    group.sample_size(10);
    group.bench_function("solmate", |b| {
        let mut cmd = prj.spark_command();
        cmd.arg("test");
        b.iter(|| {
            cmd.ensure_execute_success().unwrap();
        });
    });
}

criterion_group!(benches, spark_test_benchmark);
criterion_main!(benches);
