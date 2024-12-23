# kirin

Kirin (麒麟) - A simple tool for immersing yourself in Japanese and Chinese. Just as the mythological Kirin appears to the worthy and brings enlightenment, this app aims to make language immersion accessible and enriching.


## Setup

For UI components:

1. TailwindCSS for styling (utility-first CSS framework)
1. shadcn/ui for pre-built components (looks modern, highly customizable)


### Tauri


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

### Shadcn/ui

[ui.shadcn installation/vite](https://ui.shadcn.com/docs/installation/vite)

```
pnpm add -D tailwindcss postcss autoprefixer  # Save package to your `devDependencies`
pnpm dlx tailwindcss init -p                  # Creates tailwind.config.js and postcss.config.js
```

Added the following to [kirin/src/App.css](./kirin/src/App.css):
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

We added `"./index.html", "./src/**/*.{ts,tsx,js,jsx}"` to the `content` field in `tailwind.config.js`.

```json
    /* Shadcn/ui changes as per https://ui.shadcn.com/docs/installation/vite */
    "baseUrl": ".",
    "paths": {
        "@/*": ["./src/*"]
    }
```

```
pnpm add -D @types/node
```

```ts
import path from "path"
export default defineConfig(async () => ({
  plugins: [react()],
  
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

  ...
}));
```

When running `pnpm dlx shadcn@latest init`:

```
...
✔ Which style would you like to use? › Default
✔ Which color would you like to use as the base color? › Slate
✔ Would you like to use CSS variables for theming? … no / yes
...
```

The Default style:

1. Has better readability and contrast - important for a language learning app where users will be reading a lot of text.
1. Uses more neutral colors that won't compete with any text highlighting or language-specific features you might add later
1. Is slightly more minimalist, which helps keep focus on the content
1. Has been tested more extensively as it's the original style
1. The New York style is more decorative and has some nice flourishes, but for an app focused on language learning, the 1. Default style's clarity would be more beneficial.

Slate provides excellent contrast for text readability, which is crucial for your app

1. It has a slightly cooler tone that's easy on the eyes during long reading sessions
1. It works well with both light and dark modes
1. It provides enough visual hierarchy without being too stark (like Gray) or too warm (like Stone)
1. The differences between these colors are subtle, but Slate tends to be the most versatile and comfortable for text-heavy applications. It's also commonly used in modern documentation sites and educational platforms.

CSS variables for theming:

1. It will make theme customization much easier - users might want different themes for Chinese vs Japanese content
1. CSS variables make it easier to implement dark/light mode switching
1. You can dynamically change colors with JavaScript if needed (useful for highlighting or emphasis in language learning)
1. It's more maintainable - you can update colors in one place instead of throughout your styles
1. Better performance compared to utility class changes for theming
1. The only downside is a tiny bit more setup, but shadcn-ui handles most of that for you. The benefits for a language learning app where visual differentiation might be important definitely outweigh any minimal setup cost.

These values are registered in `components.json`.