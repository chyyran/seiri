const child_proc = require("child_process");
const isDev = require('electron-is-dev');
const path = require("path");

const launch_watcher = (onStdErr, onQuit) => {
  let watcher = child_proc.spawn((isDev
    ? "seiri-watcher"
    : `${path.join(__dirname, "../app.asar.unpacked/seiri-watcher")}`), [], {
    stdio: "pipe"
  });
  watcher.addListener("close", onQuit);
  watcher.stderr.addListener("data", onStdErr);
  watcher.stdout.pipe(process.stdout);
  watcher.stderr.pipe(process.stdout);
  return {
    quit: () => watcher.stdin.write("exit\r\n"),
    disconnect: () => {
      if (watcher) {
        try {
          watcher.disconnect();
        } catch (e) {
          // do nothing.
        }
      }
    }
  };
};

module.exports = launch_watcher;
