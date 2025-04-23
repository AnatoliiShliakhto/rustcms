# RUST CMS

<p><span style="color:red">IN DEVELOPMENT</span></p>

## Description
Content management system written in [Rust](https://www.rust-lang.org) with 
[WASM](https://developer.mozilla.org/en-US/docs/WebAssembly) `UI`
and [cross-platform](https://en.wikipedia.org/wiki/Cross-platform) application

## Components 

>**SERVER** `back-end` \
**standalone server without additional dependencies**
>>[Axum](https://axum.rs) - modular web framework built with Tokio, Tower, and Hyper \
>>[SurrealDB](https://surrealdb.com) - no-sql `embedded`, `local` or `remote` database \
>>[Utoipa](https://utoipa.rs) - OpenAPI documentation generator

>**UI** `front-end` \
**user interface based on `WASM`**
>>[Dioxus](https://dioxuslabs.com) - is the Rust framework for building fullstack web, desktop, and mobile apps \
>>[Tailwind](https://tailwindcss.com) - CSS framework for rapid UI development \
>>[DaisyUI](https://daisyui.com) - open source **Tailwind CSS component library**

>**APPLICATION** `windows` `android` etc. \
**cross-platform application**
>>[Tauri **v2.0**](https://v2.tauri.app) - cross-platform app framework written in Rust

 
## Getting started

### Prerequirements

>[LLVM](https://llvm.org) -OR- [Microsoft build tools](https://visualstudio.microsoft.com) - The LLVM Compiler Infrastructure \
>[CMAKE](https://cmake.org) - cross-platform, open-source build automation tool \
>[mkcert](https://github.com/FiloSottile/mkcert) - simple tool for creating local trusted development certificates 

1. Make localhost certificate for development purposes with [mkcert](https://github.com/FiloSottile/mkcert)
```bash
mkcert -install
mkcert localhost
```
2. Rename `localhost.pem` to `ssl.crt` and `localhost-key.pem` to `private.key` and place them in `./publish/data/cert` folder

3. Server configuration file `server-config.json` -OR- `.env` file -OR- `Environment variables` 

### Development...

### Start SurrealDB
```bash
surreal start --log info --user root --password root --bind 0.0.0.0:9000 rocksdb://./publish/data/db
```
### Compile TailwindCSS assets
```bash
npx tailwindcss -i ./rustcms-ui/input.css -o ./rustcms-ui/resources/css/main.css --minify
```
