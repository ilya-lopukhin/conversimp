# convertsimp

A **simp**le media **convert**er. Thin FFMPEG GUI wrapper built with (tauri)[https://v2.tauri.app/].

In essence, it accepts a list of files via dragging them inside the application window,
or by selecting via native file picker dialog. After that the conversion process can be started
using the "Convert" button.

An output configuration dialog lets you pass a template for the ffmpeg output format you need.

This list of files is then iterated inside the rust backend,
spawning a thread for each path to run the (ffmpeg-sidecar)[https://crates.io/crates/ffmpeg-sidecar] command.

Command exitcode is then used to notify the frontend via tauri event,
where it is used to interactively show the conversion status in a table of files.

After all files are converted, you can reset it using "Start over" button.

# building & running

There is no official release yet, so to build&run this yourself, you need:


1. (Tauri v2 build prerequisites)[https://v2.tauri.app/start/prerequisites/]
2. Node v22

Then you can follow this script to build it:

```
npm i
npm run tauri build
```

Alternatively, you can run it in development mode:

```
npm run tauri dev
```


# License

MIT or whatever
