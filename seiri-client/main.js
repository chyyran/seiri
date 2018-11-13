const { app, BrowserWindow, Menu, Tray } = require('electron')
const notifier = require('node-notifier')
const path = require('path')
const child = require('child_process')
const url = require('url')
const watcher = require('./watcher')
const isDev = require('electron-is-dev')
const appId = 'moe.chyyran.seiri'
const opn = require('opn')
const ensureConfig = require('./ensureConfig')
const autoUpdater = require('electron-updater').autoUpdater

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let win = null
let winClose = null
let tray = null
let runningWatcher = null
let watcherShouldQuit = false
const newTracksAdded = []

const gotLock = app.requestSingleInstanceLock()

app.on('second-instance', (commandLine, workingDirectory) => {
  // Someone tried to run a second instance, we should focus our window.
  if (winClose && win !== null) {
    clearTimeout(winClose)
    winClose = null
    win.show()
  } else if (win === null) {
    createWindow()
    win.once('ready-to-show', () => {
      win.show()
    })
  } else {
    win.show()
  }
})

if (!gotLock) {
  watcherShouldQuit = true
  app.quit()
  return
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

const expression = /^(TRACKADDED|E[A-Z]+)::(.*)$/
const twoparamexpr = /^(.*)\|\|(.*)$/

const processWatcherMessage = message => {
  let matches = expression.exec(message.trim())

  console.log('Received message: ' + message)
  let messageType = matches[1]
  let messagePayload = matches[2]
  switch (messageType) {
    case 'TRACKADDED':
      console.log('Track added...')
      console.log(messagePayload)
      let trackdata = twoparamexpr.exec(messagePayload)
      newTracksAdded.push(trackdata[1] + ' - ' + trackdata[2])
      break
    case 'EMISSINGTAG':
      console.log('Missing tag...')
      let tagdata = twoparamexpr.exec(messagePayload)
      notifier.notify({
        title: 'Track is missing tag.',
        message:
          'Track ' + trackdata[1] + ' is missing the ' + trackdata[2] + ' tag.',
        appID: appId
      })
      break
    case 'ETRACKMOVE':
      console.log('Track move error...')
      notifier.notify({
        title: 'Error when moving track',
        message: 'Error occurred when moving ' + messagePayload,
        appID: appId
      })
      break
    case 'ETRACK':
      console.log('track..')
      notifier.notify({
        title: 'Track error occurred.',
        message: messagePayload,
        appID: appId
      })
      break
    case 'ECREATEDIRECTORY':
      console.log('Unable to create directory...')
      notifier.notify({
        title: 'Unable to create folder.',
        message: 'Unable to create folder ' + messagePayload,
        appID: appId
      })
      break
    case 'ENONTRACK':
      console.log('non track..')
      notifier.notify({
        title: 'Non-track file found.',
        message: messagePayload + ' is not a track.',
        appID: appId
      })
      break
    case 'EWATCHERDIED':
      console.log('watcher died...')
      notifier.notify({
        title: 'Track watcher restarting.',
        message: 'Restarting the track watcher due to an error.',
        appID: appId
      })
      break
    case 'EWATCHERRESTART':
      console.log('watcher restart...')
      notifier.notify({
        title: 'Track watcher restarting.',
        message: 'Restarting the track watcher.',
        appID: appId
      })
      break
    case 'EWATCHERNOACCESS':
      console.log('watcher no access...')
      notifier.notify({
        title: 'Can not access the track library folder.',
        message:
          'The track library folder can not be accessed. Ensure it exists and then restart the track watcher.',
        appID: appId
      })
      if (runningWatcher) {
        runningWatcher.quit()
      }
      break
    case 'EWATCHER':
      console.log('watcher restart...')
      notifier.notify({
        title: 'Track watcher error.',
        message: 'Unknown track watcher error occurred.',
        appID: appId
      })
      break
    case 'ECONFIGINVALID':
      console.log('watcher restart...')
      notifier.notify({
        title: 'Configuration error.',
        message:
          'The configuration file is invalid. Fix it then restart the track watcher.',
        appID: appId
      })
      if (runningWatcher) {
        runningWatcher.quit()
      }
      break
    case 'ECONFIGIO':
      console.log('watcher restart...')
      notifier.notify({
        title: 'Configuration error.',
        message: 'Can not write to configuration path ' + messagePayload + '.',
        appID: appId
      })
      if (runningWatcher) {
        runningWatcher.quit()
      }
      break
    default:
      notifier.notify({
        title: 'Error occurred.',
        message: messagePayload + ': ' + messageType,
        appID: appId
      })
      break
  }
}

const startNewTracksNotifier = () => {
  setInterval(() => {
    if (newTracksAdded.length === 1) {
      notifier.notify({
        title: 'New Tracks Added',
        message: 'Added ' + newTracksAdded[0],
        appID: appId
      })
      newTracksAdded.length = 0
    } else if (newTracksAdded.length !== 0) {
      notifier.notify({
        title: 'New Tracks Added',
        message:
          'Added ' +
          newTracksAdded[0] +
          ' and ' +
          (newTracksAdded.length - 1) +
          ' more.',
        appID: appId
      })
      newTracksAdded.length = 0
    }
  }, 60000)
}

const restartWatcher = () => {
  console.log('Starting watcher...')
  runningWatcher = watcher(
    chunk => {
      processWatcherMessage(chunk.toString('utf8'))
    },
    chunk => {
      if (!watcherShouldQuit) {
        console.log('Watcher unexpectedly quit, restarting...')
        restartWatcher()
      }
    }
  )
}

app.on('ready', () => {
  console.log('App Ready!')
  ensureConfig(app.getPath('appData'), app.getPath('home'))
  autoUpdater.checkForUpdatesAndNotify()
  console.log('config ensured.')
  Menu.setApplicationMenu(null)
  restartWatcher()
  startNewTracksNotifier()
  tray = new Tray(__dirname + '/branding/seiri.png')
  const contextMenu = Menu.buildFromTemplate([
    {
      label: 'Show Library',
      click: () => {
        console.log('Show')
        if (winClose && win !== null) {
          clearTimeout(winClose)
          winClose = null
          win.show()
        } else if (win === null) {
          createWindow()
          win.once('ready-to-show', () => {
            win.show()
          })
        } else {
          win.show()
        }
      }
    },
    {
      label: 'Restart Track Watcher',
      click: () => {
        if (runningWatcher) {
          runningWatcher.quit()
          console.log('Sent exit signal to running watcher...')
        }
      }
    },
    { type: 'separator' },
    {
      label: 'Open Configuration Directory',
      click: () => {
        const seiriPath = path.join(app.getPath('appData'), '.seiri/')
        child.spawn('explorer', [seiriPath], { detached: true })
      }
    },
    { type: 'separator' },
    { label: 'Quit', role: 'quit' }
  ])
  tray.setToolTip('seiri is running.')
  tray.setContextMenu(contextMenu)
  contextMenu.on('menu-will-show', () => {
    console.log('menu will show!')
  })
  tray.on('click', () => {
    console.log('Restored!')
    if (winClose && win !== null) {
      clearTimeout(winClose)
      winClose = null
      win.show()
    } else if (win === null) {
      createWindow()
      win.once('ready-to-show', () => {
        win.show()
      })
    } else {
      win.show()
    }
  })
})

function createWindow() {
  // Create the browser window.
  win = new BrowserWindow({
    width: 1000,
    height: 600,
    frame: false,
    icon: __dirname + '/branding/seiri.png',
    show: false,
    webSecurity: false
  })

  let dir = isDev
    ? 'http://localhost:3000'
    : `file://${path.join(
        __dirname,
        '../app.asar.unpacked/ui.asar/index.html'
      )}`
  // and load the index.html of the app.
  win.loadURL(dir)

  // Open the DevTools.
  // win.webContents.openDevTools();

  win.on('hide', () => {
    console.log('Closing in 60 seconds...')
    winClose = setTimeout(() => {
      console.log('window destroyed.')
      win.close()
      win = null
    }, 60000)
  })

  // Emitted when the window is closed.
  win.on('closed', () => {
    // Dereference the window object, usually you would store windows
    // in an array if your app supports multi windows, this is the time
    // when you should delete the corresponding element.
    win = null
  })
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on('ready', () => {
  // Do nothing.
})

app.on('quit', () => {
  watcherShouldQuit = true
  if (runningWatcher) {
    runningWatcher.quit()
    console.log('Sent exit signal to running watcher...')
    runningWatcher.disconnect()
    console.log('Disconnected from running watcher.')
  }
})

app.on('activate', () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (win === null) {
    createWindow()
    win.once('ready-to-show', () => {
      win.show()
    })
  }
})

app.on('window-all-closed', () => {})

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and require them here.
