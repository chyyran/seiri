const { app, BrowserWindow, Menu, Tray } = require("electron");
const notifier = require("node-notifier");
const path = require("path");
const child = require("child_process");
const watcher = require("./watcher");
const isDev = require("electron-is-dev");
const appId = "moe.chyyran.seiri";
const ensureConfig = require("./ensureConfig");
const autoUpdater = require("electron-updater").autoUpdater;
const log = require("electron-log");

log.transports.file.level = "info";

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let win = null;
let winClose = null;
let tray = null;
let runningWatcher = null;
let watcherShouldQuit = false;
const newTracksAdded = [];

const gotLock = app.requestSingleInstanceLock();

app.on("second-instance", (commandLine, workingDirectory) => {
  // Someone tried to run a second instance, we should focus our window.
  if (winClose && win !== null) {
    clearTimeout(winClose);
    winClose = null;
    win.show();
  } else if (win === null) {
    createWindow();
    win.once("ready-to-show", () => {
      win.show();
    });
  } else {
    win.show();
  }
});

if (!gotLock) {
  watcherShouldQuit = true;
  app.quit();
  return;
}

/*

| Code                          | Description                                            |
| ----------------------------- | ------------------------------------------------------ |
| `TRACKADDED(Artist||Title)`   | A track has successfully been added to the library     |
| `!ETRACK`                      | Generic track error                                    |
| !`ETRACKMOVE(Path)`            | The given track could not be moved to its library path |
| `!ECREATEDIRECTORY(Directory)` | The given directory could not be created               |
| `!ENONTRACK(Path)`             | The given path is not a track                          |
| !`EMISSINGTAG(Track||Tag)`     | The given track is missing the given tag               |
| !`EWATCHER`                    | Generic watcher error                                  |
| !`EWATCHERDIED`                | The watcher died                                       |
| !`EWATCHERNOACCESS(Path)`      | The watcher can not access the given folder            |
| `ECONFIGINVALID`              | The configuration file is invalid                      |
| `ECONFIGIO(Path)`             | The given configuration path can not be accessed       |
*/

const expression = /^(TRACKADDED|E[A-Z]+)::(.*)$/;
const twoparamexpr = /^(.*)\|\|(.*)$/;

const processWatcherMessage = message => {
  try {
    if (message === null || message === undefined || !!!message) {
      log.warn("bad recv <" + _message + ">");
      return;
    }

    let _message = message.trim();
    let matches = expression.exec(_message);
    log.info("MsgRecv <" + _message + ">");

    if (!!!matches || (!!matches && matches.length !== 3)) {
      log.warn("bad recv <" + _message + ">");
      return;
    }
    let messageType = matches[1];
    let messagePayload = matches[2];

    switch (messageType) {
      case "TRACKADDED":
        log.info("TRACKADDED recv with payload <" + messagePayload + ">");
        let trackdata = twoparamexpr.exec(messagePayload);
        if (trackdata && trackdata.length === 3) {
          newTracksAdded.push(trackdata[1] + " - " + trackdata[2]);
        } else {
          log.warn("TRACKADDED bad recv <" + _message + ">");
        }
        break;
      case "EMISSINGTAG":
        log.info("EMISSINGTAG recv with payload <" + messagePayload + ">");
        let tagdata = twoparamexpr.exec(messagePayload);
        if (tagdata && tagdata.length === 3) {
          notifier.notify({
            title: "Track is missing tag.",
            message:
              "Track " + tagdata[1] + " is missing the " + tagdata[2] + " tag.",
            appID: appId
          });
        } else {
          log.info("EMISSINGTAG bad recv <" + _message + ">");
        }
        break;
      case "ETRACKMOVE":
        log.info("ETRACKMOVE recv");
        notifier.notify({
          title: "Error when moving track",
          message: "Error occurred when moving " + messagePayload,
          appID: appId
        });
        break;
      case "ETRACK":
        log.info("ETRACK recv");
        notifier.notify({
          title: "Track error occurred.",
          message: messagePayload,
          appID: appId
        });
        break;
      case "ECREATEDIRECTORY":
        log.info("ECREATEDIRECTORY recv");
        notifier.notify({
          title: "Unable to create folder.",
          message: "Unable to create folder " + messagePayload,
          appID: appId
        });
        break;
      case "ENONTRACK":
        log.info("ENONTRACK recv");
        notifier.notify({
          title: "Non-track file found.",
          message: messagePayload + " is not a track.",
          appID: appId
        });
        break;
      case "EWATCHERDIED":
        log.info("EWATCHERDIED recv");
        notifier.notify({
          title: "Track watcher restarting.",
          message: "Restarting the track watcher due to an error.",
          appID: appId
        });
        break;
      case "EWATCHERRESTART":
        log.info("EWATCHERRESTART recv");
        notifier.notify({
          title: "Track watcher restarting.",
          message: "Restarting the track watcher.",
          appID: appId
        });
        break;
      case "EWATCHERNOACCESS":
        log.info("EWATCHERNOACCESS recv");
        notifier.notify({
          title: "Can not access the track library folder.",
          message:
            "The track library folder can not be accessed. Ensure it exists and then restart the track watcher.",
          appID: appId
        });
        if (runningWatcher) {
          runningWatcher.quit();
        }
        break;
      case "EWATCHER":
        log.info("EWATCHER recv");
        notifier.notify({
          title: "Track watcher error.",
          message: "Unknown track watcher error occurred.",
          appID: appId
        });
        break;
      case "ECONFIGINVALID":
        log.info("ECONFIGINVALID recv");
        notifier.notify({
          title: "Configuration error.",
          message:
            "The configuration file is invalid. Fix it then restart the track watcher.",
          appID: appId
        });
        if (runningWatcher) {
          runningWatcher.quit();
        }
        break;
      case "ECONFIGIO":
        log.info("ECONFIGIO recv");
        notifier.notify({
          title: "Configuration error.",
          message:
            "Can not write to configuration path " + messagePayload + ".",
          appID: appId
        });
        if (runningWatcher) {
          runningWatcher.quit();
        }
        break;
      default:
        log.warn("EUNKNOWN recv");

        notifier.notify({
          title: "Error occurred.",
          message: messagePayload + ": " + messageType,
          appID: appId
        });
        break;
    }
  } catch(err) {
    log.warn("bad err recv <" + _message + ">");
  }
};

const startNewTracksNotifier = () => {
  setInterval(() => {
    if (newTracksAdded.length === 1) {
      notifier.notify({
        title: "New Tracks Added",
        message: "Added " + newTracksAdded[0],
        appID: appId
      });
      newTracksAdded.length = 0;
    } else if (newTracksAdded.length !== 0) {
      notifier.notify({
        title: "New Tracks Added",
        message:
          "Added " +
          newTracksAdded[0] +
          " and " +
          (newTracksAdded.length - 1) +
          " more.",
        appID: appId
      });
      newTracksAdded.length = 0;
    }
  }, 60000);
};

const restartWatcher = () => {
  log.info("Starting watcher...");
  runningWatcher = watcher(
    chunk => {
      processWatcherMessage(chunk.toString("utf8"));
    },
    chunk => {
      if (!watcherShouldQuit) {
        log.info("Watcher unexpectedly quit, restarting...");
        restartWatcher();
      }
    }
  );
};

app.on("ready", () => {
  log.info("App Ready!");
  ensureConfig(app.getPath("appData"), app.getPath("home"));
  autoUpdater.checkForUpdatesAndNotify();
  log.info("config ensured.");
  Menu.setApplicationMenu(null);
  restartWatcher();
  startNewTracksNotifier();
  tray = new Tray(__dirname + "/branding/seiri.png");
  const contextMenu = Menu.buildFromTemplate([
    {
      label: "Show Library",
      click: () => {
        log.info("Show");
        if (winClose && win !== null) {
          clearTimeout(winClose);
          winClose = null;
          win.show();
        } else if (win === null) {
          createWindow();
          win.once("ready-to-show", () => {
            win.show();
          });
        } else {
          win.show();
        }
      }
    },
    {
      label: "Restart Track Watcher",
      click: () => {
        if (runningWatcher) {
          runningWatcher.quit();
          log.info("Sent exit signal to running watcher...");
        }
      }
    },
    { type: "separator" },
    {
      label: "Open Configuration Directory",
      click: () => {
        const seiriPath = path.join(app.getPath("appData"), ".seiri/");
        child.spawn("explorer", [seiriPath], { detached: true });
      }
    },
    { type: "separator" },
    { label: "Quit", role: "quit" }
  ]);
  tray.setToolTip("seiri is running.");
  tray.setContextMenu(contextMenu);
  contextMenu.on("menu-will-show", () => {
    log.info("menu will show!");
  });
  tray.on("click", () => {
    log.info("Restored!");
    if (winClose && win !== null) {
      clearTimeout(winClose);
      winClose = null;
      win.show();
    } else if (win === null) {
      createWindow();
      win.once("ready-to-show", () => {
        win.show();
      });
    } else {
      win.show();
    }
  });
});

function createWindow() {
  // Create the browser window.
  win = new BrowserWindow({
    width: 1000,
    height: 600,
    frame: false,
    icon: path.join(__dirname, "branding", "seiri.png"),
    show: false,
    webSecurity: false,
    webPreferences: {
      contextIsolation: true,
      enableRemoteModule: true,
      preload: path.join(__dirname , "preload.js")
    },
  });

  let dir = isDev
    ? "http://localhost:3000"
    : `file://${path.join(
        __dirname,
        "../app.asar.unpacked/ui.asar/index.html"
      )}`;
  // and load the index.html of the app.
  win.loadURL(dir);

  // Open the DevTools.
  if (isDev) {
    win.webContents.openDevTools();
  }

  win.on("hide", () => {
    log.info("Closing in 60 seconds...");
    winClose = setTimeout(() => {
      log.info("window destroyed.");
      win.close();
      win = null;
    }, 60000);
  });

  // Emitted when the window is closed.
  win.on("closed", () => {
    // Dereference the window object, usually you would store windows
    // in an array if your app supports multi windows, this is the time
    // when you should delete the corresponding element.
    win = null;
  });
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on("ready", () => {
  // Do nothing.
});

app.on("quit", () => {
  watcherShouldQuit = true;
  if (runningWatcher) {
    runningWatcher.quit();
    log.info("Sent exit signal to running watcher...");
    runningWatcher.disconnect();
    log.info("Disconnected from running watcher.");
  }
});

app.on("activate", () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (win === null) {
    createWindow();
    win.once("ready-to-show", () => {
      win.show();
    });
  }
});

app.on("window-all-closed", () => {});

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and require them here.
