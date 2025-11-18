<?php

declare(strict_types=1);

namespace WilliamThogersen\Minifier;

use Exception;
use Throwable;

/**
 * Exception thrown when HTML minification fails
 *
 * Provides detailed error information from the Rust FFI library including:
 * - Error type classification (null pointer, invalid UTF-8, internal error)
 * - Detailed error messages
 * - Context about what operation failed
 */
class MinifierException extends Exception
{
    /**
     * Error codes matching MinifierError enum from Rust
     */
    public const ERROR_SUCCESS = 0;
    public const ERROR_NULL_POINTER = 1;
    public const ERROR_INVALID_UTF8 = 2;
    public const ERROR_INTERNAL_ERROR = 3;

    /**
     * Additional context about where the error occurred
     */
    private ?string $context = null;

    /**
     * The input that caused the error (truncated if too long)
     */
    private ?string $inputSnippet = null;

    public function __construct(
        string $message = "",
        int $code = 0,
        ?Throwable $previous = null,
        ?string $context = null,
        ?string $input = null
    ) {
        parent::__construct($message, $code, $previous);
        $this->context = $context;

        // Store a snippet of the input for debugging (max 200 chars)
        if ($input !== null) {
            $this->inputSnippet = mb_strlen($input) > 200
                ? mb_substr($input, 0, 200) . '...'
                : $input;
        }
    }

    /**
     * Check if this is a null pointer error
     */
    public function isNullPointerError(): bool
    {
        return $this->code === self::ERROR_NULL_POINTER;
    }

    /**
     * Check if this is an invalid UTF-8 error
     */
    public function isInvalidUtf8Error(): bool
    {
        return $this->code === self::ERROR_INVALID_UTF8;
    }

    /**
     * Check if this is an internal error
     */
    public function isInternalError(): bool
    {
        return $this->code === self::ERROR_INTERNAL_ERROR;
    }

    /**
     * Get the error type as a human-readable string
     */
    public function getErrorType(): string
    {
        return match ($this->code) {
            self::ERROR_NULL_POINTER => 'Null Pointer Error',
            self::ERROR_INVALID_UTF8 => 'Invalid UTF-8 Error',
            self::ERROR_INTERNAL_ERROR => 'Internal Error',
            default => 'Unknown Error',
        };
    }

    /**
     * Get the context where the error occurred
     */
    public function getContext(): ?string
    {
        return $this->context;
    }

    /**
     * Get a snippet of the input that caused the error
     */
    public function getInputSnippet(): ?string
    {
        return $this->inputSnippet;
    }

    /**
     * Get a detailed error report
     */
    public function getDetailedMessage(): string
    {
        $parts = [];

        $parts[] = "MinifierException [{$this->getErrorType()}]";

        if ($this->context !== null) {
            $parts[] = "Context: {$this->context}";
        }

        $parts[] = "Message: {$this->message}";

        if ($this->inputSnippet !== null) {
            $parts[] = "Input snippet: {$this->inputSnippet}";
        }

        return implode("\n", $parts);
    }

    /**
     * Convert to string with full details
     */
    public function __toString(): string
    {
        return $this->getDetailedMessage() . "\n" . parent::__toString();
    }
}
