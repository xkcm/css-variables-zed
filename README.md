# CSS Variables (LSP) for Zed

Project-wide CSS custom properties (variables) support for Zed, port form
[`css-variables-language-server`](https://github.com/vunguyentuan/vscode-css-variables).

## Features

- Project-wide indexing of `--var`s defined in `.css`, `.scss`, `.sass`, `.less`
- Autocomplete and color previews for `var(--...)`
- Hover and go to definition for CSS variables
- Works across HTML / JS / TS / Svelte / Vue etc. where `var(--...)` is used

## Requirements (not needed starting from 0.0.2)

- Node.js
- `css-variables-language-server` installed globally:

```bash
npm i -g css-variables-language-server
```

## Known limitations

- Only CSS/SCSS/LESS/SASS files are scanned; variables defined inside HTML <style> blocks are not indexed (this matches upstream behaviour)
- If a variable is defined in multiple files, the last scanned definition wins
- Currently the server is a global npm installation; future versions may bundle
