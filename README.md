# cpar
**A commmand line utility that allows for batch coping & renaming with globs.**

## Help Page
```
Copies and mass-renames files using the variable character ($)

Usage: cpar <INPUT_PATH> <OUTPUT_PATH>

Arguments:
  <INPUT_PATH>   The files to be copied and renamed
  <OUTPUT_PATH>  The path for the renamed files

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Example

### Making new copies with different names
Assume a folder with the following files:
```
src/
	- ContactForm.tsx
	- ContactForm.module.scss
	- ContactForm.astro
```

`cpar src/ContactForm$ src/MembershipForm$` the folder will contain:

```
src/
	- ContactForm.tsx
	- ContactForm.module.scss
	- ContactForm.astro
	+ MembershipForm.tsx (same contents as ContactForm.tsx)
	+ MembershipForm.module.scss (same contents as ContactForm.module.scss)
	+ MembershipForm.astro (same contents as ContactForm.astro)
```

### Making new copies with different extensions
Assume a folder with the following files:
```
src/
	- foo.ts
	- bar.ts
```

`cpar src/$.js src/$.ts` the folder will contain:

```
src/
	- foo.js
	- bar.js
```

### Test Command
`cargo test -- --test-threads=1`

## Use Cases
Probably a bunch, but the ones that prodded me to make the tool in the first place were:
- Build scripts for multi-language projects
- Porting large numbers of HTML templates to `.astro` files
- Creating a copy of a web UI component to use as a starter template

## Todos for V1 
- Implement a proper Rust testing library - the included tests are rudimentary and use the standard library.
- Change tests such that they can run in parallel (currently they only run consecutively)
- Find a way to use the actual Bash wildcard character (`*`) in place of (`$`).