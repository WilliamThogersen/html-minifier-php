<?php

declare(strict_types=1);

namespace WilliamThogersen\Minifier;

/**
 * Configuration options for HTML minification
 */
class MinifierOptions
{
    public function __construct(
        public bool $removeComments = true,
        public bool $collapseWhitespace = true,
        public bool $removeOptionalTags = true,
        public bool $removeAttributeQuotes = true,
        public bool $collapseBooleanAttributes = true,
        public bool $removeDefaultAttributes = true,
        public bool $removeEmptyAttributes = true,
        public bool $minifyJs = true,
        public bool $minifyCss = true,
        public bool $preserveConditionalComments = false,
    ) {
    }

    /**
     * Create default options (aggressive minification)
     */
    public static function default(): self
    {
        return new self();
    }

    /**
     * Create conservative options (safer but less aggressive)
     */
    public static function conservative(): self
    {
        return new self(
            removeComments: true,
            collapseWhitespace: true,
            removeOptionalTags: false,
            removeAttributeQuotes: false,
            collapseBooleanAttributes: true,
            removeDefaultAttributes: false,
            removeEmptyAttributes: false,
            minifyJs: true,
            minifyCss: true,
            preserveConditionalComments: true,
        );
    }

    /**
     * Create minimal options (only safe transformations)
     */
    public static function minimal(): self
    {
        return new self(
            removeComments: true,
            collapseWhitespace: true,
            removeOptionalTags: false,
            removeAttributeQuotes: false,
            collapseBooleanAttributes: false,
            removeDefaultAttributes: false,
            removeEmptyAttributes: false,
            minifyJs: false,
            minifyCss: false,
            preserveConditionalComments: true,
        );
    }

    /**
     * Create options from FFI CMinifierOptions struct
     *
     * @internal
     */
    public static function fromFFI(object $cOptions): self
    {
        return new self(
            removeComments: $cOptions->remove_comments,
            collapseWhitespace: $cOptions->collapse_whitespace,
            removeOptionalTags: $cOptions->remove_optional_tags,
            removeAttributeQuotes: $cOptions->remove_attribute_quotes,
            collapseBooleanAttributes: $cOptions->collapse_boolean_attributes,
            removeDefaultAttributes: $cOptions->remove_default_attributes,
            removeEmptyAttributes: $cOptions->remove_empty_attributes,
            minifyJs: $cOptions->minify_js,
            minifyCss: $cOptions->minify_css,
            preserveConditionalComments: $cOptions->preserve_conditional_comments,
        );
    }

    /**
     * Create a copy with some options modified
     */
    public function with(
        ?bool $removeComments = null,
        ?bool $collapseWhitespace = null,
        ?bool $removeOptionalTags = null,
        ?bool $removeAttributeQuotes = null,
        ?bool $collapseBooleanAttributes = null,
        ?bool $removeDefaultAttributes = null,
        ?bool $removeEmptyAttributes = null,
        ?bool $minifyJs = null,
        ?bool $minifyCss = null,
        ?bool $preserveConditionalComments = null,
    ): self {
        return new self(
            removeComments: $removeComments ?? $this->removeComments,
            collapseWhitespace: $collapseWhitespace ?? $this->collapseWhitespace,
            removeOptionalTags: $removeOptionalTags ?? $this->removeOptionalTags,
            removeAttributeQuotes: $removeAttributeQuotes ?? $this->removeAttributeQuotes,
            collapseBooleanAttributes: $collapseBooleanAttributes ?? $this->collapseBooleanAttributes,
            removeDefaultAttributes: $removeDefaultAttributes ?? $this->removeDefaultAttributes,
            removeEmptyAttributes: $removeEmptyAttributes ?? $this->removeEmptyAttributes,
            minifyJs: $minifyJs ?? $this->minifyJs,
            minifyCss: $minifyCss ?? $this->minifyCss,
            preserveConditionalComments: $preserveConditionalComments ?? $this->preserveConditionalComments,
        );
    }
}
