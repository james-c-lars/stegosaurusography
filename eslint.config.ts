/* eslint-disable @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment,@typescript-eslint/no-unsafe-call,@typescript-eslint/no-unsafe-argument */
import js from "@eslint/js";
import ts from "typescript-eslint";
import svelte from "eslint-plugin-svelte";
import svelteConfig from "./svelte.config.js";
import prettier from "eslint-plugin-prettier/recommended";
import { includeIgnoreFile } from "@eslint/compat";
import path from "node:path";
import { fileURLToPath } from "node:url";

const tsConfig = ts.config({
    files: ["**/*.js", "**/*.cjs", "**/*.ts", "**/*.svelte"],
    extends: [js.configs.recommended, ...ts.configs.strictTypeChecked, ...ts.configs.stylisticTypeChecked, prettier],
    languageOptions: {
        parserOptions: {
            project: ["tsconfig.json", "tsconfig.node.json"],
            tsconfigRootDir: import.meta.dirname,
            extraFileExtensions: [".svelte"],
        },
    },
});

const svelteEslintConfig = ts.config({
    files: ["**/*.svelte"],
    extends: [...svelte.configs["flat/recommended"], ...svelte.configs["flat/prettier"]],
    languageOptions: {
        parserOptions: {
            parser: ts.parser,
            svelteConfig,
        },
    },
});

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const gitignorePath = path.resolve(__dirname, ".gitignore");
const tauriGitignorePath = path.resolve(__dirname, "src-tauri/.gitignore");

export default ts.config(
    includeIgnoreFile(gitignorePath),
    includeIgnoreFile(tauriGitignorePath),
    ...tsConfig,
    ...svelteEslintConfig,
);
