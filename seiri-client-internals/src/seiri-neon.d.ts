import { Track } from "./types";

interface Seiri {
    queryTracks: (bang: string) => { tracks: Track[] };
    refreshTracks: (filePaths: string[]) => void;
    openTrackFolder: (track: Track) => void;
    hideWindow: () => void;
}

declare global {
    interface Window {
        seiri: Seiri
    }
}
