import js from "@eslint/js";
import ts from "typescript-eslint";
import svelte from "eslint-plugin-svelte";
import svelteConfig from "./svelte.config.js";
import prettier from "eslint-plugin-prettier/recommended";
import * as fs from "node:fs";

const tsConfig = ts.config({
    files: ["**/*.js", "**/*.ts", "**/*.svelte"],
    // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
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
    // eslint-disable-next-line @typescript-eslint/no-unsafe-member-access,@typescript-eslint/no-unsafe-assignment
    extends: [...svelte.configs["flat/recommended"], ...svelte.configs["flat/prettier"]],
    languageOptions: {
        parserOptions: {
            parser: ts.parser,
            svelteConfig,
        },
    },
});

/**
 * Prepends directory to the entry
 * @param entry {string} The entry to prepend directory to
 * @param directory {string} Directory path
 * @return {string}
 */
function prependDirectoryToEntry(entry, directory) {
    if (directory === "") return entry;

    return entry.startsWith("!") ? `!${directory}/${entry.substring(1)}` : `${directory}/${entry}`;
}

/**
 * Loads an ignore file as an array of strings
 * @param path {string} The path to the file to load
 * @return {string[]}
 */
function loadIgnore(path) {
    const pathFragments = path.split("/");
    pathFragments.pop();
    const directory = pathFragments.join("/");

    return fs
        .readFileSync(path)
        .toString()
        .split("\n")
        .map((entry) => entry.trim())
        .filter((entry) => !entry.startsWith("#"))
        .map((entry) => prependDirectoryToEntry(entry, directory));
}

// noinspection JSUnusedGlobalSymbols
export default ts.config(...tsConfig, ...svelteEslintConfig, {
    ignores: [...loadIgnore(".gitignore"), ...loadIgnore("src-tauri/.gitignore")],
});
