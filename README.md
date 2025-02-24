# convertsimp

A **simp**le media **convert**er. Thin FFMPEG GUI wrapper built with [tauri](https://v2.tauri.app/).

In essence, it accepts a list of files via dragging them inside the application window,
or by selecting via native file picker dialog. After that the conversion process can be started
using the "Convert" button.

An output configuration dialog lets you pass a template for the ffmpeg output format you need.

This list of files is then iterated inside the rust backend,
spawning a thread for each path to run the [ffmpeg-sidecar](https://crates.io/crates/ffmpeg-sidecar) command.

Command exitcode is then used to notify the frontend via tauri event,
where it is used to interactively show the conversion status in a table of files.

After all files are converted, you can reset it using "Start over" button.

![Screenshot](https://i.imgur.com/UibGBDo.png)

# building & running

There is no official release yet, so to build&run this yourself, you need:


1. [Tauri v2 build prerequisites](https://v2.tauri.app/start/prerequisites/)
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

After initial launch, it will download the FFMPEG binary via the awesome rust [ffmpeg-sidecar](https://github.com/nathanbabcock/ffmpeg-sidecar)
and save it to the appropriate `%APP_DATA%` location


# Project TODO

- [ ] ffmpeg status/missing error/download/locate binary
- [ ] convert templates in settings
- [ ] spinners for convert button while conversion is in progress, spinners for table rows in status
- [ ] file size and actions columns in table
- [ ] actions: delete if status is pending, locate if done, locate logs if error
- [ ] write logs for each conversion next to binary, stdout stderr
- [ ] build for x64, test on intel mac
- [ ] build for linux, test on ubuntu
- [ ] build for windows, test on bootstrap

# License

MIT or whatever
