# kirin

Kirin (麒麟) - A simple tool for immersing yourself in Japanese and Chinese. Just as the mythological Kirin appears to the worthy and brings enlightenment, this app aims to make language immersion accessible and enriching.


## Setup

For UI components:

1. TailwindCSS for styling (utility-first CSS framework)
1. shadcn/ui for pre-built components (looks modern, highly customizable)



[Tauri](https://v2.tauri.app/start/)

First, update Rust toolchains and rustup:
```
rustup update
```

Then install the tauri CLI:
```
cargo install create-tauri-app --locked
```

And create a project:
```
cargo create-tauri-app kirin
```

We used the following configuratoin:
```
➜  ~/src/github.com/seafoodfry/kirin git:(init) cargo create-tauri-app kirin
✔ Identifier · com.kirin.app
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm, deno, bun)
✔ Choose your package manager · pnpm
✔ Choose your UI template · React - (https://react.dev/)
✔ Choose your UI flavor · TypeScript

Template created!

Your system is missing dependencies (or they do not exist in $PATH):
╭─────────┬───────────────────────────╮
│ Node.js │ Visit https://nodejs.org/ │
╰─────────┴───────────────────────────╯

Make sure you have installed the prerequisites for your OS: https://tauri.app/start/prerequisites/, then run:
  cd kirin
  pnpm install
  pnpm tauri android init
  pnpm tauri ios init

For Desktop development, run:
  pnpm tauri dev

For Android development, run:
  pnpm tauri android dev

For iOS development, run:
  pnpm tauri ios dev
```

We are missing nodeJS so we will use the
[Node Version Manager NVCM CLI](https://github.com/nvm-sh/nvm)

As of now, see https://nodejs.org/en/about/previous-releases, NodeJS 22 is the current active LTS.
So we will use it:
```
nvm install 22
```

To use it:
```
nvm use 22
```

Note that we are using
[pnpm: the fast, disk space efficient package manager](https://pnpm.io/).
To install it run:
```
cd kirin
corepack enable pnpm
```

```
corepack use pnpm@latest
```

This approach has several benefits over `npm install -g pnpm`:

1. Keeps pnpm isolated to your project
1. Ensures consistent package manager version across team members
1. Avoids polluting your global system
1. The pnpm version can be tracked in your package.json

> This will add a `"packageManager"` field in your local package.json which will instruct Corepack to always use a
> specific version on that project. This can be useful if you want reproducability, as all developers who are using
> Corepack will use the same version as you. When a new version of pnpm is released, you can re-run the above command.
> xref: [pnpm/installation#using-corepack](https://pnpm.io/installation#using-corepack)

At this point you should be able to run
```
pnpm install
```

And at this point you should be able to run
```
pnpm tauri dev
```