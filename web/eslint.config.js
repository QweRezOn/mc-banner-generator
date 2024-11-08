import stylistic from "@stylistic/eslint-plugin"
import eslintPluginSvelte from "eslint-plugin-svelte"
import eslint from "@eslint/js"
import tseslint from "typescript-eslint"
import globals from "globals"
import svelteParser from "svelte-eslint-parser"

const customized = stylistic.configs.customize({
    indent: 4,
    quotes: "double",
    semi: false,
})

export default tseslint.config(
    eslint.configs.recommended,
    ...tseslint.configs.stylistic,
    ...eslintPluginSvelte.configs["flat/recommended"],
    {
        languageOptions: {
            globals: {
                ...globals.browser,
            },
            parser: svelteParser,
            parserOptions: {
                parser: tseslint.parser,
                extraFileExtensions: [".svelte"],
            },
        },
        plugins: {
            "@stylistic": stylistic,
        },
        rules: {
            ...customized.rules,
            "svelte/indent": ["error", { indent: 4 }],
            "@stylistic/max-len": ["warn", 120],
            "@stylistic/no-trailing-spaces": ["error", { skipBlankLines: true }],
            "@stylistic/member-delimiter-style": ["error", {
                multiline: {
                    delimiter: "comma",
                    requireLast: true,
                },
                singleline: {
                    delimiter: "comma",
                    requireLast: false,
                },
            }],
            "@stylistic/multiline-ternary": "off",
            "@stylistic/brace-style": ["error", "1tbs", {
                allowSingleLine: true,
            }],
            "@typescript-eslint/no-unused-vars": [
                "error",
                { argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
            ],
        },
    },
)
