use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use html_minifier_ffi::{
    minify_css, minify_html_tokens, minify_html_with_options, minify_javascript, MinifierOptions,
};

// Sample HTML for benchmarking
const SMALL_HTML: &str = r#"<div class="container">
    <p>Hello World!</p>
</div>"#;

const MEDIUM_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test Page</title>
    <style>
        body { margin: 0; padding: 0; }
        .container { max-width: 1200px; margin: 0 auto; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Welcome</h1>
        <p>This is a test page with some content.</p>
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ul>
    </div>
    <script>
        console.log('Hello World');
        function test() { return 42; }
    </script>
</body>
</html>"#;

const LARGE_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Large Test Page</title>
    <style>
        /* Reset styles */
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        .header { background: #4CAF50; color: white; padding: 1rem; }
        .nav { background: #333; }
        .nav ul { list-style: none; display: flex; }
        .nav li { padding: 1rem; }
        .content { padding: 2rem 0; }
        .footer { background: #333; color: white; text-align: center; padding: 2rem; }
    </style>
</head>
<body>
    <header class="header">
        <div class="container">
            <h1>Large Test Page</h1>
        </div>
    </header>
    <nav class="nav">
        <div class="container">
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/services">Services</a></li>
                <li><a href="/contact">Contact</a></li>
            </ul>
        </div>
    </nav>
    <main class="content">
        <div class="container">
            <article>
                <h2>Article Title</h2>
                <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
                <p>Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.</p>
                <h3>Subsection</h3>
                <ul>
                    <li>Feature 1: High performance</li>
                    <li>Feature 2: Easy to use</li>
                    <li>Feature 3: Well documented</li>
                    <li>Feature 4: Active community</li>
                </ul>
                <table>
                    <thead>
                        <tr>
                            <th>Name</th>
                            <th>Value</th>
                            <th>Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>Item 1</td>
                            <td>100</td>
                            <td>Active</td>
                        </tr>
                        <tr>
                            <td>Item 2</td>
                            <td>200</td>
                            <td>Pending</td>
                        </tr>
                        <tr>
                            <td>Item 3</td>
                            <td>300</td>
                            <td>Complete</td>
                        </tr>
                    </tbody>
                </table>
            </article>
        </div>
    </main>
    <footer class="footer">
        <div class="container">
            <p>&copy; 2024 Test Company. All rights reserved.</p>
        </div>
    </footer>
    <script>
        // Initialize application
        (function() {
            console.log('Application initialized');

            function setupEventListeners() {
                document.querySelectorAll('.nav a').forEach(link => {
                    link.addEventListener('click', function(e) {
                        console.log('Navigating to:', e.target.href);
                    });
                });
            }

            function loadData() {
                return fetch('/api/data')
                    .then(response => response.json())
                    .then(data => {
                        console.log('Data loaded:', data);
                        return data;
                    })
                    .catch(error => {
                        console.error('Error loading data:', error);
                    });
            }

            document.addEventListener('DOMContentLoaded', function() {
                setupEventListeners();
                loadData();
            });
        })();
    </script>
</body>
</html>"#;

const JAVASCRIPT: &str = r#"
function calculateTotal(items) {
    let total = 0;
    for (let i = 0; i < items.length; i++) {
        total += items[i].price * items[i].quantity;
    }
    return total;
}

const processData = async (data) => {
    const result = await fetch('/api/process', {
        method: 'POST',
        body: JSON.stringify(data)
    });
    return result.json();
};

// Regular expression example
const pattern = /^[a-zA-Z0-9]+$/;
const division = 10 / 2;
"#;

const CSS: &str = r#"
/* Global styles */
body {
    margin: 0;
    padding: 0;
    font-family: Arial, sans-serif;
    line-height: 1.6;
    color: #333;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
}

.button {
    display: inline-block;
    padding: 10px 20px;
    background-color: #4CAF50;
    color: white;
    text-decoration: none;
    border-radius: 4px;
}

.button:hover {
    background-color: #45a049;
}
"#;

fn benchmark_html_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("html_small");
    group.throughput(Throughput::Bytes(SMALL_HTML.len() as u64));

    group.bench_function("default_options", |b| {
        b.iter(|| minify_html_tokens(black_box(SMALL_HTML)))
    });

    group.bench_function("conservative_options", |b| {
        let options = MinifierOptions::conservative();
        b.iter(|| minify_html_with_options(black_box(SMALL_HTML), black_box(&options)))
    });

    group.finish();
}

fn benchmark_html_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("html_medium");
    group.throughput(Throughput::Bytes(MEDIUM_HTML.len() as u64));

    group.bench_function("default_options", |b| {
        b.iter(|| minify_html_tokens(black_box(MEDIUM_HTML)))
    });

    group.finish();
}

fn benchmark_html_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("html_large");
    group.throughput(Throughput::Bytes(LARGE_HTML.len() as u64));

    group.bench_function("default_options", |b| {
        b.iter(|| minify_html_tokens(black_box(LARGE_HTML)))
    });

    group.finish();
}

fn benchmark_javascript(c: &mut Criterion) {
    let mut group = c.benchmark_group("javascript");
    group.throughput(Throughput::Bytes(JAVASCRIPT.len() as u64));

    group.bench_function("minify", |b| b.iter(|| minify_javascript(black_box(JAVASCRIPT))));

    group.finish();
}

fn benchmark_css(c: &mut Criterion) {
    let mut group = c.benchmark_group("css");
    group.throughput(Throughput::Bytes(CSS.len() as u64));

    group.bench_function("minify", |b| b.iter(|| minify_css(black_box(CSS))));

    group.finish();
}

fn benchmark_attribute_heavy_html(c: &mut Criterion) {
    let html = r#"<div class="container main wrapper" id="main-content" data-id="123" data-name="test" style="color: red; margin: 10px; padding: 5px;">
        <button type="submit" class="btn btn-primary btn-large" disabled="disabled" data-toggle="modal">Click me</button>
        <input type="text" name="username" id="username" placeholder="Enter username" required="required" autocomplete="off">
    </div>"#;

    let mut group = c.benchmark_group("attribute_heavy");
    group.throughput(Throughput::Bytes(html.len() as u64));

    group.bench_function("minify", |b| b.iter(|| minify_html_tokens(black_box(html))));

    group.finish();
}

criterion_group!(
    benches,
    benchmark_html_small,
    benchmark_html_medium,
    benchmark_html_large,
    benchmark_javascript,
    benchmark_css,
    benchmark_attribute_heavy_html
);

criterion_main!(benches);
