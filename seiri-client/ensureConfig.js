const path = require("path");
const fs = require('fs');
const { dialog } = require('electron');
const toml = require('toml-js')

const ensureConfig = (directory, home) => {

    const seiriPath = path.join(directory, ".seiri/");
    const musicPath = path.join(home, "Music", "seiri");
    console.log(seiriPath);
    if (!fs.existsSync(seiriPath)) {
        fs.mkdirSync(seiriPath);
    }
    const configPath = path.join(seiriPath, "config.toml");
    console.log(fs.existsSync(configPath));
    if (!fs.existsSync(configPath)) {
        console.log("Choosing music folder...");
        const watchPathChoice = dialog.showOpenDialog({
            title: 'Choose music folder',
            defaultPath: musicPath,
            properties: ['openDirectory']
        });
        let watchPath = watchPathChoice ? watchPathChoice[0] : musicPath;
        watchPath = watchPath.replace(new RegExp("\\\\", 'g'), "\\\\");
        console.log('chose ' + watchPath)
        const configData = toml.dump({music_folder: watchPath});
        fs.writeFileSync(configPath, configData);
    }
}

module.exports = ensureConfig;