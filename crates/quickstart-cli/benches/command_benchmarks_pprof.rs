//! Profiling benchmarks for command execution in cargo-quickstart using pprof
//!
//! Run with: `cargo bench --bench command_benchmarks_pprof`

// Allow specific lints for benchmarks only
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::disallowed_methods)]
#![allow(clippy::uninlined_format_args)]
#![allow(unused_imports)]

use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::Output;
use pprof::criterion::PProfProfiler;

// Benchmark listing available templates
fn bench_list_templates(c: &mut Criterion) {
    c.bench_function("list_templates (pprof)", |b| {
        b.iter(|| {
            // Find templates directory
            let template_dir = quickstart_lib::find_templates_dir().unwrap();
            // List available templates
            let loader = quickstart_lib::template::TemplateLoader::new(template_dir);
            let templates = loader
                .list_templates(
                    quickstart_lib::ProjectType::Binary,
                    quickstart_lib::template::TemplateVariant::Minimal,
                )
                .unwrap();
            criterion::black_box(templates)
        });
    });
}

criterion_group! {
    name = command_benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_list_templates
}
criterion_main!(command_benches);
