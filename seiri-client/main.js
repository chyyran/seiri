const { app, BrowserWindow, Menu, Tray } = require("electron");
const notifier = require("node-notifier");
const path = require("path");
const url = require("url");
const watcher = require("./watcher");

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let win = null;
let winClose = null;
let tray = null;
let runningWatcher = null;
let isQuitting = false;
const newTracksAdded = [];

const processWatcherMessage = (message) => {
  let messageType = message.split('~')[0];
  let messagePayload = message.slice(message.indexOf("~") + 1);
  let messageMessage = messagePayload.split(message.indexOf(":") + 1)[0];
  switch (messageType) {
    case "TRACKMOVEERR":
      console.log("Track move error...")
      notifier.notify({
        title: "Track Move Error",
        message: "Error occurred when moving " + messageMessage
      })
    case "MISSINGTAG":
      console.log("Missing tag...")
      notifier.notify({
        title: "Track is missing tag.",
        message: messagePayload
      })
    case "TRACKADDED":
      console.log("Track added...");
      newTracksAdded.push(messageMessage);
      break;
    default:
      console.log("Unknown message...")
  }
}

const startNewTracksNotifier = () => {
  setInterval(() => {
    if (newTracksAdded.length === 1) {
      notifier.notify({
        title: "New Tracks Added",
        message: "Added " + newTracksAdded[0]
      })
      newTracksAdded.length = 0;
    } else if (newTracksAdded.length !== 0) {
      notifier.notify({
        title: "New Tracks Added",
        message: "Added " + newTracksAdded[0] + " and " + (newTracksAdded.length - 1) + " more."
      })
      newTracksAdded.length = 0;
    }
  }, 60000)
}

const restartWatcher = () => {
  console.log("Starting watcher...");
  runningWatcher = watcher(
    chunk => {
      processWatcherMessage(chunk.toString("utf8"))
    },
    chunk => {
      if (!isQuitting) {
        console.log("Watcher unexpectedly quit, restarting...");
        restartWatcher();
      }
    }
  );
};

app.on("ready", () => {
  Menu.setApplicationMenu(null);
  restartWatcher();
  startNewTracksNotifier();
  tray = new Tray(__dirname + '/branding/seiri.png')
  const contextMenu = Menu.buildFromTemplate([
    {
      label: 'Show Library', click: () => {
        console.log("Show");
        if (winClose && win !== null) {
          clearTimeout(winClose);
          winClose = null;
          win.show();
        } else if (win === null) {
          createWindow();
          win.once('ready-to-show', () => {
            win.show()
          })
        } else {
          win.show();
        }
      }
    },
    {
      label: 'Restart Folder Watcher', click: () => {
        if (runningWatcher) {
          runningWatcher.quit();
          console.log("Sent exit signal to running watcher...");
        }
      }
    },
    { type: 'separator' },
    { label: 'Quit', role: 'quit' },
  ])
  tray.setToolTip('seiri is running.')
  tray.setContextMenu(contextMenu);
  contextMenu.on('menu-will-show', () => {
    console.log('menu will show!');
  })
  tray.on('click', () => {
    console.log('Restored!');
    if (winClose && win !== null) {
      clearTimeout(winClose);
      winClose = null;
      win.show();
    } else if (win === null) {
      createWindow();
      win.once('ready-to-show', () => {
        win.show();
      })
    } else {
      win.show();
    }
  });
});

function createWindow() {
  // Create the browser window.
  win = new BrowserWindow({ width: 800, height: 600, frame: false, icon: __dirname + '/branding/seiri.png', show: false });

  // and load the index.html of the app.
  win.loadURL(
    url.format({
      pathname: "localhost:3000",
      protocol: "http:",
      slashes: true
    })
  );

  // Open the DevTools.
  //win.webContents.openDevTools();

  win.on("hide", () => {
    console.log("Closing in 60 seconds...");
    winClose = setTimeout(() => {
      console.log("window destroyed.");
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
  isQuitting = true;
  if (runningWatcher) {
    runningWatcher.quit();
    console.log("Sent exit signal to running watcher...");
    runningWatcher.disconnect();
    console.log("Disconnected from running watcher.");
  }
});

app.on("activate", () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (win === null) {
    createWindow();
    win.once('ready-to-show', () => {
      win.show()
    })
  }
});

app.on('window-all-closed', () => {

});

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and require them here.
