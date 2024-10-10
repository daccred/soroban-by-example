# Soroban-by-example Contributing Guide

Hi! I'm really excited that you are interested in contributing to Soroban-by-example. 
Before submitting your contribution, please make sure to take a moment and read through the following guidelines:

- [Code of Conduct](https://github.com/daccred/soroban-by-example/blob/dev/.github/CODE_OF_CONDUCT.md)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)

## Pull Request Guidelines

- **DO NOT** checkin `dist` in the commits.

- It's OK to have multiple small commits as you work on the PR - GitHub will automatically squash it before merging.

- Make sure `pnpm build` passes. (see [development setup](#development-setup))

- If adding a new contract:

  - Test, test, test before sending a pr.

## Development Setup

You will need [Node.js](https://nodejs.org) **version 18+** and [pnpm](https://pnpm.io/) **version 8+**.

After cloning the repo, run:

```bash
$ pnpm i # install the dependencies of the project
```

### Commonly used PNPM scripts

```bash
# watch and auto re-build
$ pnpm run dev

# build all dist files, including npm packages
$ pnpm build
```

## Project Structure

- **`pages`**: contains the example content and frontmatter. Adding a new example means you will create a new file and save it with the example name. However, it would be 

- **`snippets`**: contains soroban code samples.

- **`sidebar.ts`**: These handles the layout of the sidebar. 

## Credits

Thank you to all the people who have already contributed to SorobanExamples!

<a href="https://github.com/daccred/soroban-by-example/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=daccred/soroban-by-example" />
</a>