import { ITrack } from "./types";

// tslint:disable-next-line:interface-name
export interface UpdateQuery {
    type: "UPDATE_QUERY",
    query: string
}

// tslint:disable-next-line:interface-name
export interface UpdateTracks {
    type: "UPDATE_TRACKS",
    tracks: ITrack[]
}

export type SeiriAction = UpdateQuery | UpdateTracks;