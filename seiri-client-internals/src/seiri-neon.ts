import ElectronWindow from "./ElectronWindow";
import { Track } from "./types";

declare var window : ElectronWindow;

const seiriInstance = window.require<Seiri>("seiri-neon");

interface Seiri {
    queryTracks: (bang: string) => { tracks: Track[] };
    refreshTracks: (filePaths: string[]) => void;
}

export default seiriInstance;

