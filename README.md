# Tauri example app

> In this example we use Tauri to create a simple desktop app from a boilerplate Next.js app.

This is a sample app showing some basic abilities of Tauri app:

1. **Sidecar**. A Go sidecar which starts a local TCP server and waits for a connection from the Tauri app.
2. **Asset rendering**. The Tauri app can work with your local files, for example, to render them.

## Install dependencies

> Use [https://github.com/jdx/mise](https://github.com/jdx/mise)

```bash
mise install
```

## Run the app (dev)

```bash
# Install dependencies
yarn install

# Build the sidecar app
cd src-tauri/sidecar
make install

# Run the desktop app (dev)
cd -
yarn tauri dev
```
