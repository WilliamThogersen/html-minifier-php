<?php

declare(strict_types=1);

namespace WilliamThogersen\Minifier;

use FFI;
use RuntimeException;

/**
 * HTMLMinifier - High-performance HTML minifier using Rust FFI
 *
 * Features:
 * - Fast HTML, CSS, and JavaScript minification
 * - Configurable minification options
 * - Proper error handling with detailed error messages
 * - UTF-8 aware processing
 */
class HTMLMinifier
{
    private FFI $ffi;
    private ?MinifierOptions $defaultOptions = null;
    private static ?string $version = null;

    /**
     * Error code indicating success (matches MinifierError enum from Rust)
     */
    private const ERROR_SUCCESS = 0;

    /**
     * Maximum input size in bytes (default: 50MB)
     * Can be overridden in constructor
     */
    private int $maxInputSize;

    /**
     * Whether to validate UTF-8 encoding
     */
    private bool $validateUtf8;

    public function __construct(
        ?string $libraryPath = null,
        int $maxInputSize = 50 * 1024 * 1024, // 50MB default
        bool $validateUtf8 = true
    ) {
        $this->maxInputSize = $maxInputSize;
        $this->validateUtf8 = $validateUtf8;

        if ($libraryPath === null) {
            $libraryPath = self::detectLibraryPath();
        }

        $this->ffi = FFI::cdef(
            "
            typedef struct {
                bool remove_comments;
                bool collapse_whitespace;
                bool remove_optional_tags;
                bool remove_attribute_quotes;
                bool collapse_boolean_attributes;
                bool remove_default_attributes;
                bool remove_empty_attributes;
                bool minify_js;
                bool minify_css;
                bool preserve_conditional_comments;
            } CMinifierOptions;

            typedef enum {
                Success = 0,
                NullPointer = 1,
                InvalidUtf8 = 2,
                InternalError = 3,
            } MinifierError;

            char* minify_html_string(const char* html_ptr);
            char* minify_html_string_with_options(const char* html_ptr, CMinifierOptions options);
            void free_string(char* ptr);
            MinifierError minifier_get_last_error();
            char* minifier_get_last_error_message();
            void minifier_clear_error();
            CMinifierOptions minifier_options_default();
            CMinifierOptions minifier_options_conservative();
            char* minifier_get_version();
            ",
            $libraryPath
        );

        // Verify version compatibility
        $this->verifyLibraryVersion();
    }

    /**
     * Verify that the Rust library version matches the PHP wrapper version
     *
     * @throws RuntimeException if versions don't match
     */
    private function verifyLibraryVersion(): void
    {
        $versionPtr = $this->ffi->minifier_get_version();

        if ($versionPtr === null) {
            throw new RuntimeException(
                'Failed to get library version. The Rust library may be incompatible.'
            );
        }

        $libraryVersion = FFI::string($versionPtr);
        $this->ffi->free_string($versionPtr);

        $phpVersion = self::getVersion();

        // Check major.minor compatibility (allow patch differences)
        $phpVersionParts = explode('.', $phpVersion);
        $libVersionParts = explode('.', $libraryVersion);

        $phpMajorMinor = $phpVersionParts[0] . '.' . ($phpVersionParts[1] ?? '0');
        $libMajorMinor = $libVersionParts[0] . '.' . ($libVersionParts[1] ?? '0');

        if ($phpMajorMinor !== $libMajorMinor) {
            throw new RuntimeException(
                sprintf(
                    'Version mismatch: PHP wrapper is v%s but Rust library is v%s. ' .
                    'Please rebuild the library with: bash build.sh',
                    $phpVersion,
                    $libraryVersion
                )
            );
        }
    }

    /**
     * Create a new instance (alternative to using constructor directly)
     */
    public static function create(
        ?string $libraryPath = null,
        int $maxInputSize = 50 * 1024 * 1024,
        bool $validateUtf8 = true
    ): self {
        return new self($libraryPath, $maxInputSize, $validateUtf8);
    }

    /**
     * Get the package version from composer.json
     */
    public static function getVersion(): string
    {
        if (self::$version !== null) {
            return self::$version;
        }

        $version = null;

        if (class_exists('Composer\InstalledVersions')) {
            try {
                $version = \Composer\InstalledVersions::getVersion('williamthogersen/minifier');
            } catch (\OutOfBoundsException $e) {
                // Package not found, fall through to file reading
            }
        }

        if ($version === null) {
            $composerPath = dirname(__DIR__) . '/composer.json';
            if (file_exists($composerPath)) {
                $composerData = json_decode(file_get_contents($composerPath), true);
                if (isset($composerData['version'])) {
                    $version = $composerData['version'];
                }
            }
        }

        // Default fallback if version can't be determined
        if ($version === null) {
            $version = '1.0.0';
        }

        // Normalize version (Composer may add extra .0, e.g., "1.0.0.0")
        // Keep only major.minor.patch
        $parts = explode('.', $version);
        if (count($parts) > 3) {
            $version = implode('.', array_slice($parts, 0, 3));
        }

        self::$version = $version;
        return self::$version;
    }

    /**
     * Detect the library path based on OS and architecture
     */
    private static function detectLibraryPath(): string
    {
        $baseDir = __DIR__ . '/Lib/';
        $baseName = 'libhtml_minifier_ffi';

        // Detect architecture
        $arch = php_uname('m'); // e.g., 'x86_64', 'arm64', 'aarch64'

        // Map PHP architecture to Rust target names
        $targets = [];
        if (PHP_OS_FAMILY === 'Darwin') {
            if (in_array($arch, ['arm64', 'aarch64'])) {
                $targets = ['aarch64-apple-darwin', 'x86_64-apple-darwin'];
            } else {
                $targets = ['x86_64-apple-darwin', 'aarch64-apple-darwin'];
            }
            $extensions = ['dylib'];
        } elseif (PHP_OS_FAMILY === 'Linux') {
            if (in_array($arch, ['arm64', 'aarch64'])) {
                $targets = ['aarch64-unknown-linux-gnu', 'x86_64-unknown-linux-gnu'];
            } else {
                $targets = ['x86_64-unknown-linux-gnu', 'aarch64-unknown-linux-gnu'];
            }
            $extensions = ['so'];
        } else {
            // Windows
            $targets = ['x86_64-pc-windows-msvc'];
            $extensions = ['dll'];
        }

        // Try architecture-specific libraries first
        foreach ($targets as $target) {
            foreach ($extensions as $ext) {
                $path = "{$baseDir}{$baseName}.{$target}.{$ext}";
                if (file_exists($path)) {
                    return $path;
                }
            }
        }

        // Fallback to generic names (backwards compatibility)
        foreach ($extensions as $ext) {
            $path = "{$baseDir}{$baseName}.{$ext}";
            if (file_exists($path)) {
                return $path;
            }
        }

        throw new RuntimeException(
            "Could not find HTML minifier library for {$arch} on " . PHP_OS_FAMILY . ". " .
            "Searched in: {$baseDir} for targets: " . implode(', ', $targets)
        );
    }

    /**
     * Set default options for all minify calls
     */
    public function setDefaultOptions(MinifierOptions $options): void
    {
        $this->defaultOptions = $options;
    }

    /**
     * Validate input HTML before processing
     *
     * @throws MinifierException if validation fails
     */
    private function validateInput(string $html): void
    {
        // Check for empty input
        if ($html === '') {
            return; // Empty input is valid, just returns empty
        }

        // Check input size
        $inputSize = strlen($html);
        if ($inputSize > $this->maxInputSize) {
            throw new MinifierException(
                message: sprintf(
                    'Input size (%d bytes) exceeds maximum allowed size (%d bytes)',
                    $inputSize,
                    $this->maxInputSize
                ),
                code: 0,
                previous: null,
                context: 'Input validation',
                input: substr($html, 0, 200)
            );
        }

        // Validate UTF-8 encoding if enabled
        if ($this->validateUtf8 && !mb_check_encoding($html, 'UTF-8')) {
            throw new MinifierException(
                message: 'Input contains invalid UTF-8 sequences',
                code: 0,
                previous: null,
                context: 'UTF-8 validation',
                input: substr($html, 0, 200)
            );
        }
    }

    /**
     * Get the last error message from the Rust library
     */
    private function getLastErrorMessage(): ?string
    {
        $errorPtr = $this->ffi->minifier_get_last_error_message();
        if ($errorPtr === null) {
            return null;
        }

        $message = FFI::string($errorPtr);
        $this->ffi->free_string($errorPtr);

        return $message;
    }

    /**
     * Check for errors and throw exception if needed
     */
    private function checkError(?string $context = null, ?string $input = null): void
    {
        $error = $this->ffi->minifier_get_last_error();

        if ($error !== self::ERROR_SUCCESS) {
            $message = $this->getLastErrorMessage() ?? 'Unknown error';

            $this->ffi->minifier_clear_error();

            throw new MinifierException(
                message: $message,
                code: $error,
                previous: null,
                context: $context,
                input: $input
            );
        }
    }

    /**
     * Minify HTML with default options
     *
     * @throws MinifierException if input validation fails or minification error occurs
     */
    public function minify(string $html, ?MinifierOptions $options = null): string
    {
        // Validate input first
        $this->validateInput($html);

        // Handle empty input
        if ($html === '') {
            return '';
        }

        // Use provided options, or default options, or library defaults
        $options = $options ?? $this->defaultOptions;

        if ($options === null) {
            return $this->minifySimple($html);
        }

        return $this->minifyWithOptions($html, $options);
    }

    /**
     * Minify HTML using simple default settings
     *
     * Note: Errors are cleared at the FFI boundary to prevent
     * leakage between requests in PHP-FPM environments
     */
    private function minifySimple(string $html): string
    {
        $minifiedPtr = $this->ffi->minify_html_string($html);

        if ($minifiedPtr === null) {
            $this->checkError('Failed to minify HTML', $html);
            throw new MinifierException(
                message: 'Minification returned null without error',
                code: 0,
                previous: null,
                context: 'minifySimple',
                input: $html
            );
        }

        $minified = FFI::string($minifiedPtr);
        $this->ffi->free_string($minifiedPtr);

        return $minified;
    }

    /**
     * Minify HTML with custom options
     */
    private function minifyWithOptions(string $html, MinifierOptions $options): string
    {
        $cOptions = $this->ffi->new('CMinifierOptions');
        $cOptions->remove_comments = $options->removeComments;
        $cOptions->collapse_whitespace = $options->collapseWhitespace;
        $cOptions->remove_optional_tags = $options->removeOptionalTags;
        $cOptions->remove_attribute_quotes = $options->removeAttributeQuotes;
        $cOptions->collapse_boolean_attributes = $options->collapseBooleanAttributes;
        $cOptions->remove_default_attributes = $options->removeDefaultAttributes;
        $cOptions->remove_empty_attributes = $options->removeEmptyAttributes;
        $cOptions->minify_js = $options->minifyJs;
        $cOptions->minify_css = $options->minifyCss;
        $cOptions->preserve_conditional_comments = $options->preserveConditionalComments;

        $minifiedPtr = $this->ffi->minify_html_string_with_options($html, $cOptions);

        if ($minifiedPtr === null) {
            $this->checkError('Failed to minify HTML with options', $html);
            throw new MinifierException(
                message: 'Minification returned null without error',
                code: 0,
                previous: null,
                context: 'minifyWithOptions',
                input: $html
            );
        }

        $minified = FFI::string($minifiedPtr);
        $this->ffi->free_string($minifiedPtr);

        return $minified;
    }

    /**
     * Get default minifier options
     */
    public function getDefaultOptions(): MinifierOptions
    {
        $cOptions = $this->ffi->minifier_options_default();
        return MinifierOptions::fromFFI($cOptions);
    }

    /**
     * Get conservative minifier options (safer but less aggressive)
     */
    public function getConservativeOptions(): MinifierOptions
    {
        $cOptions = $this->ffi->minifier_options_conservative();
        return MinifierOptions::fromFFI($cOptions);
    }
}
