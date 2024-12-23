# kamekai

Kamekai is an interactive tool for immersing yourself in Japanese and Chinese.

# Setup

For UI components:

1. TailwindCSS for styling (utility-first CSS framework)
1. shadcn/ui for pre-built components (looks modern, highly customizable)

---
## Tauri


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
cargo create-tauri-app kamekai
```

We used the following configuratoin:
```
➜  ~/src/github.com/seafoodfry/kamekai git:(init) cargo create-tauri-app kamekai
✔ Identifier · com.kamekai.app
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
  cd kamekai
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
cd kamekai
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

---
## Shadcn/ui

The instructions we followed are based on
[ui.shadcn installation/vite](https://ui.shadcn.com/docs/installation/vite).

```
pnpm add -D tailwindcss postcss autoprefixer  # Save package to your `devDependencies`
pnpm dlx tailwindcss init -p                  # Creates tailwind.config.js and postcss.config.js
```

Added the following to [kamekai/src/App.css](./kamekai/src/App.css):
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

We added `"./index.html", "./src/**/*.{ts,tsx,js,jsx}"` to the `content` field in `tailwind.config.js`.

And we added the following snippet to `tsconfig.json` and `tsconfig.node.json`:
```json
    /* Shadcn/ui changes as per https://ui.shadcn.com/docs/installation/vite */
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
```

We ran the following:
```
pnpm add -D @types/node
```

And added this to `vite.config.ts`:
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


At this point, we updated the styling of the app to use Tailwindcss and Shadcnui.

We added these components:
```
pnpm dlx shadcn@latest add input button
```

```js
<div className="min-h-screen bg-[#1E1E1E] text-white flex flex-col items-center justify-start">
```

- `min-h-screen`: Makes the div at least as tall as the viewport (100vh)
- `bg-[#1E1E1E]`: Sets a dark background color (using a custom hex value)
- `text-white`: Sets text color to white
- `flex`: Makes it a flex container
- `flex-col`: Sets flex direction to column (stacks children vertically)
- `items-center`: Centers children horizontally (cross-axis alignment)
- `justify-start`: Aligns children to the start vertically (main-axis alignment)

```js
<main className="w-full max-w-3xl px-4 py-24 text-center">
```

- `w-full`: Width 100% of parent
- `max-w-3xl`: Maximum width of 48rem (768px)
- `px-4`: Padding left and right of 1rem (16px)
- `py-24`: Padding top and bottom of 6rem (96px)
- `text-center`: Centers text content

```js
<h1 className="text-5xl font-bold mb-20">
```

- `text-5xl`: Font size of 3rem (48px)
- `font-bold`: Font weight of 700
- `mb-20`: Margin bottom of 5rem (80px)

```js
<div className="flex justify-center items-center space-x-8 mb-20">
```

- `flex`: Makes it a flex container
- `justify-center`: Centers children horizontally
- `items-center`: Centers children vertically
- `space-x-8`: Adds 2rem (32px) spacing between children horizontally
- `mb-20`: Margin bottom of 5rem (80px)


```js
<a className="hover:scale-110 transition-transform">
```

- `hover:scale-110`: On hover, scales the element to 110% of its size
- `transition-transform`: Makes the scale transformation smooth`

```js
<img className="logo vite w-32 h-32">
```

- `logo`: Custom class (defined in CSS)
- `w-32`: Width of 8rem (128px)
- `h-32`: Height of 8rem (128px)

```js
<form className="flex justify-center gap-4"/>
```

- `gap-4`: adds spacing between elements in a flex or grid container. 4 units in Tailwind's spacing scale, which equals 1rem or 16px.

```js
className="max-w-[240px] bg-[#2F2F2F] border-gray-600 text-white placeholder-gray-400 rounded-xl focus-visible:ring-blue-500"
```

- `max-w-[240px]`: Sets a maximum width of 240 pixels
    - The square brackets `[]` allow you to use custom values instead of Tailwind's default scale
Without brackets, you'd use predefined values like `max-w-sm` or `max-w-md`
- `bg-[#2F2F2F]`: Sets the background color to a dark gray
    - Again using square brackets for a custom hex color
    - Could use Tailwind's built-in colors like `bg-gray-800` instead
- `border-gray-600`: Sets the border color to gray
    - Uses Tailwind's gray color scale (from 50 to 900)
    - 600 is a medium-dark gray
- `text-white`: Makes the text color white
- `placeholder-gray-400`: Sets the placeholder text color to a light gray
    - Only affects the placeholder text ("Enter a name...")
    - 400 is a lighter shade in the gray scale
- `rounded-xl`: Makes the corners rounded
    - xl means "extra large" corner radius
    - You could use `rounded-lg` (large), `rounded-md` (medium), etc.
- `focus-visible:ring-blue-500`: When the input is focused
    - `focus-visible`: is a state modifier that applies styles when the element is focused
    - `ring-blue-500` adds a blue outline ring
    - 500 is a medium shade of blue

```js
<p className="text-xl mt-8">{greetMsg}</p>
```

- `mt-8` in Tailwind CSS means "margin top" with a size of 8 units in Tailwind's spacing scale.

The sizing system in Tailwind works in multiples of 4 pixels by default:

4 = 1rem (16px)
8 = 2rem (32px)
16 = 4rem (64px)
20 = 5rem (80px)
24 = 6rem (96px)
32 = 8rem (128px)

adjust the roundness by using different values:

- `rounded` for slight rounding
- `rounded-lg` for more rounding
- `rounded-xl` for even more rounding
- `rounded-2xl` for very round corners
- `rounded-full` for fully rounded (pill shape)

---
## Text Processing View

```
pnpm dlx shadcn@latest add card
```