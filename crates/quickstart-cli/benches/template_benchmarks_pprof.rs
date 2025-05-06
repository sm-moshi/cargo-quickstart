//! Profiling benchmarks for template functionality in cargo-quickstart using pprof
//!
//! Run with: `cargo bench --bench template_benchmarks_pprof`

// Allow specific lints for benchmarks only
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::disallowed_methods)]
#![allow(clippy::uninlined_format_args)]
#![allow(unused_imports)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pprof::criterion::Output;
use pprof::criterion::PProfProfiler;
use std::path::PathBuf;

// Benchmark template loading and rendering
fn bench_template_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("Template Operations");

    // Find the templates directory
    let template_dir = match quickstart_lib::find_templates_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error finding templates directory: {e}");
            return;
        }
    };

    let loader = quickstart_lib::template::TemplateLoader::new(template_dir);

    // Benchmark template listing
    group.bench_function("list_templates", |b| {
        b.iter(|| {
            black_box(
                loader
                    .list_templates(
                        quickstart_lib::ProjectType::Binary,
                        quickstart_lib::template::TemplateVariant::Minimal,
                    )
                    .unwrap(),
            )
        });
    });

    // Benchmark template loading
    group.bench_function("load_template", |b| {
        b.iter(|| {
            let template_path = "base/README.md.hbs";
            black_box(loader.load_template(template_path).unwrap())
        });
    });

    // Benchmark template existence check
    group.bench_function("template_exists", |b| {
        b.iter(|| {
            let template_path = "base/README.md.hbs";
            black_box(loader.template_exists(template_path))
        });
    });

    group.finish();
}

criterion_group! {
    name = template_benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_template_rendering
}
criterion_main!(template_benches);
