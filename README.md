# Blot Bot Desktop App
The Blot Bot desktop app is an all-inclusive designing and printing app for the Blot Bot drawing machine.
It is a visual implementation of Blot Bot Core, the core library for interfacing with Blot Bot.

The app has two major goals:
- a visual editor to create and design drawings
- a visual interface to connect to Blot Bot, to draw the drawings

## Stack
The desktop app is written using the [Tauri v2](https://v2.tauri.app/) framework, as it is a web frame.<br>
I use TypeScript in conjunction with Rust to logically control the app.<br>
I use plain old CSS with [SvelteKit](https://svelte.dev/) to markup and design the visual frontend.<br>

The Rust frontend implements `bbcore` which is my custom Blot Bot drawing design / firmware interfacing library.

![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF)
![Svelte](https://img.shields.io/badge/svelte-%23f1413d.svg?style=for-the-badge&logo=svelte&logoColor=white)
![CSS3](https://img.shields.io/badge/css3-%231572B6.svg?style=for-the-badge&logo=css3&logoColor=white)

## Screenshots
Here are four screenshots from the app which demonstrate some drawing styles from `bbcore`.
- the "Scribble" drawing method, which draws circles around points from a weighted voronoi diagram, based on an input image. (old screenshot)
- the "Entropy" drawing method, which is a purely mathematical drawing method using layered Perlin noise on a swirl.
- the "Waves" drawing method, which draws sine waves based on an input image.
- the "Islands" drawing method, which generates island/mountain like terrain using layered Perlin noise.
![Screenshot of Scribbles](https://i.imgur.com/dUKxQls.jpeg)
![Screenshot of Entropy](https://raw.githubusercontent.com/blot-bot-org/showcase/refs/heads/main/entropy.jpg)
![Screenshot of Waves](https://raw.githubusercontent.com/blot-bot-org/showcase/refs/heads/main/thegreatwave.jpg)
![Screenshot of Islands](https://raw.githubusercontent.com/blot-bot-org/showcase/refs/heads/main/islands.rs.jpg)


## Licensing
This project is currently [unlicensed](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository#choosing-the-right-license).
