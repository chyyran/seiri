import actionCreatorFactory from 'typescript-fsa';
import { asyncFactory } from 'typescript-fsa-redux-thunk';
import State from "./State";
import { Track } from "./types";


interface ElectronWindow extends Window {
    require: any;
}

declare var window: ElectronWindow;

const actionCreator = actionCreatorFactory();
const createAsync = asyncFactory<State>(actionCreator);


// tslint:disable-next-line:interface-name
export interface UpdateQuery {
    type: "UPDATE_QUERY",
    query: string
}

// tslint:disable-next-line:interface-name
export interface UpdateTracks {
    type: "UPDATE_TRACKS",
    tracks: Track[]
}
// tslint:disable-next-line:interface-name
export interface UpdateTracksTimerTick {
    type: "UPDATE_TRACKS_TICK",
}

export const updateTracks = actionCreator<{tracks: Track[]}>("UPDATE_TRACKS")

export const updateQuery = createAsync<{query: string}, {}>("UPDATE_QUERY", (query, dispatch) => {
    const tracks = window.require("seiri-neon")(query.query)
    dispatch(updateTracks(tracks))
    return { type: "UPDATE_QUERY", query }
})

export const updateTracksTick = createAsync<{}, {}>("UPDATE_TRACKS_TICK", (query, dispatch, getState) => {
    const state = getState();
    const tracks = window.require("seiri-neon")(state.query)
    // tslint:disable-next-line:no-console
    console.log("tick!")
    dispatch(updateTracks(tracks))
    window.setTimeout(() => dispatch(updateTracksTick.action()), 30000)
    return { type: "UPDATE_TRACKS_TICK" }
})
export type SeiriAction = UpdateQuery | UpdateTracks;