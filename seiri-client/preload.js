
const { contextBridge } = require('electron')
const seiri = require('seiri-neon')
const process = require('process')
const child = require("child_process");
const path = require("path");
const shell = require("electron").shell;
const remote = require("electron").remote;

contextBridge.exposeInMainWorld('seiri', {
    queryTracks: seiri.queryTracks,
    refreshTracks: seiri.refreshTracks,
    openTrackFolder: (track) => {
        if (process.platform == 'win32') {
            child.spawn("explorer", [path.dirname(track.filePath)], { detached: true });
        } else {
            shell.openExternal(path.dirname(track.filePath));
        }
    },
    hideWindow: () => remote.getCurrentWindow().hide(),
  })