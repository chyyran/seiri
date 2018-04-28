const { app, BrowserWindow, Menu, Tray } = require("electron");
const path = require("path");
const url = require("url");

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let win;
let winClose;
let tray = null

app.on('ready', () => {
  tray = new Tray('./icon.ico')
  const contextMenu = Menu.buildFromTemplate([
    {label: 'Item1', type: 'radio'},
    {label: 'Item2', type: 'radio'},
    {label: 'Item3', type: 'radio', checked: true},
    {label: 'Item4', type: 'radio'}
  ])
  tray.setToolTip('This is my application.')
  tray.setContextMenu(contextMenu);
  
  tray.on('click', () => {
    console.log('clicked!');
    if (winClose) {
      clearTimeout(winClose);
      winClose = null;
      win.show();
    } else {
      createWindow();
    }
  });
})


function createWindow() {
  // Create the browser window.
  win = new BrowserWindow({ width: 800, height: 600, frame: false });

  // and load the index.html of the app.
  win.loadURL(
    url.format({
      pathname: "localhost:3000",
      protocol: "http:",
      slashes: true
    })
  );

  // Open the DevTools.
  win.webContents.openDevTools();

  win.on("hide", () => {
    console.log("Closing in 60 seconds...");
    winClose = setTimeout(() => {
      win.close();
    }, 60000)
  })

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
app.on("ready", createWindow);

// Quit when all windows are closed.
app.on("window-all-closed", () => {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  // if (process.platform !== "darwin") {
  //   app.quit();
  // }
});

app.on("activate", () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (win === null) {
    createWindow();
  }
});

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and require them here.
