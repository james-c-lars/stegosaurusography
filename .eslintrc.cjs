/* eslint-env node */
module.exports = {
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:prettier/recommended",
    ],
    parser: "@typescript-eslint/parser",
    plugins: ["@typescript-eslint", "svelte3"],
    overrides: [
        {
            files: ["**/*.svelte"],
            processor: "svelte3/svelte3",
        },
    ],
    settings: {
        "svelte3/typescript": true,
    },
    ignorePatterns: ["src-tauri/target/**"],
    root: true,
};
