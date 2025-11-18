# HTML Minifier

[![Latest Version](https://img.shields.io/badge/Version-v1.0.0-blue)](https://github.com/WilliamThogersen/html-minifier/releases)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)




A high-performance HTML minifier for PHP, powered by a Rust backend.

This library provides a simple and fast way to minify HTML, including inline CSS and JavaScript, using a robust token-based parser instead of regular expressions. It's designed for production use, offering significant performance gains over native PHP solutions.

## Features

- **Fast:** Minification is handled by a pre-compiled Rust library, making it significantly faster than pure PHP alternatives.
- **Reliable:** Uses a token-based parser, which is more accurate and safer than regex-based solutions.
- **Easy to Use:** Simple PHP API that's easy to integrate into any project.
- **Configurable:** Comes with sensible defaults but allows for fine-grained control over the minification process.
- **Cross-Platform:** Works on Linux macOS

## Requirements

- PHP 7.4 or higher
- `ffi` PHP extension enabled

To check if the `ffi` extension is enabled, run:

```sh
php -m | grep ffi
```

If it's not listed, you'll need to enable it in your `php.ini` file:

```ini
ffi.enable=1
```

## Installation

You can install the library via Composer:

```sh
composer require WilliamThogersen/minifier
```

## Usage

### Basic Minification

```php
use WilliamThogersen\Minifier\HTMLMinifier;

$minifier = new HTMLMinifier();

$html = '
    <div class="container">
        <p style="color: red;">   Hello,   world!   </p>
    </div>
';

$minifiedHtml = $minifier->minify($html);

echo $minifiedHtml;
// Output: <div class=container><p style=color:red>Hello, world!</p></div>
```

### Using Custom Options

You can customize the minification process by passing an options object to the `minify` method.

```php
use WilliamThogersen\Minifier\HTMLMinifier;
use WilliamThogersen\Minifier\MinifierOptions;

$minifier = new HTMLMinifier();

$options = new MinifierOptions(
    removeComments: true,
    collapseWhitespace: true,
    minifyJs: false, // Don't minify inline JS
    minifyCss: false // Don't minify inline CSS
);

$minifiedHtml = $minifier->minify($html, $options);
```

### Pre-configured Option Sets

The library also provides pre-configured option sets for common use cases:

- `MinifierOptions::default()`: The default, well-balanced set of options.
- `MinifierOptions::conservative()`: A more conservative set of options that prioritizes safety over aggressive minification.
- `MinifierOptions::minimal()`: A minimal set of options that only performs the most basic and safe transformations.

```php
use WilliamThogersen\Minifier\MinifierOptions;

// Use the conservative options
$options = MinifierOptions::conservative();
$minifiedHtml = $minifier->minify($html, $options);
```


## How It Works

This library uses PHP's Foreign Function Interface (FFI) to call a pre-compiled Rust library (`.so`, `.dylib`, or `.dll`). The Rust library is responsible for the heavy lifting of parsing and minifying the HTML. This approach provides the speed of a compiled language like Rust with the convenience of a PHP library.

The compiled libraries for common architectures are included with the package, so you don't need to have Rust installed on your server.

## Development

If you want to contribute to the project, you'll need to set up the development environment.

### Building the Rust Library

The Rust library needs to be compiled from the source in the `rust/` directory. A build script is provided to simplify this process.

```sh
# Build the library for your current platform
bash build.sh
```

This will compile the Rust code and place the resulting library file in the `src/Lib/` directory.

### Running Tests

The project uses PHPUnit for testing.

```sh
# Run all tests
composer test

# Run unit tests only
composer test:unit

# Run integration tests only
composer test:integration
```

## License

The MIT License (MIT). Please see the [LICENSE](LICENSE) file for more information.
