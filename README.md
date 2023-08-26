# basic-dioxus

This is a simple template project I setup for creating a frontend web app using Rust/Dioxus with TailwindCSS. The default project generation tool gets you like 90% of the way there, but I experienced some issues with Tailwind's CSS file not getting successfully copied to the /dist directory, so I created a little scaffolding to get around this.

# Install

Start by cloning the repository:

```
git clone https://github.com/weavertron5000/basic-dioxus.git
```

If you haven't already, install Node.js and NPM. These are needed to run nodemon, which will monitor the Tailwind.css directory and will serve your app. Install nodemon using NPM:

```
npm install -g nodemon
```

Run an update using cargo to ensure all packages have been retrieved and are up to date:

```
cargo update
```

Run a build, to ensure everything builds correctly:

```
cargo build
```

# Usage

I've included helper scripts to launch a hot reload environment for both Windows and MacOS/Linux (win_start_* and bash_start_* files accordingly). To start on Windows, run win_start_tailwind.bat to get Tailwind running:

```
./win_start_tailwind.bat
```

Then run Dioxus. This batch file uses nodemon to monitor Tailwind's output directory and will reload Dioxus in the event an update is detected.

```
./win_start_dioxus.bat
```

If on MacOS or Linux, you can use the bash scripts:

```
./bash_start_tailwind
```

And:

```
./bash_start_dioxus
```


# Future

In the future, I may implement a simple mock Node.js server that serves mock data directly from .json files from disk. This would allow for rapid frontend development!