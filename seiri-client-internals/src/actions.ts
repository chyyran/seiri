import actionCreatorFactory from 'typescript-fsa';
import { asyncFactory } from 'typescript-fsa-redux-thunk';
import State from "./State";
import { Track } from "./types";

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

export interface UpdateSelected {
    type: "UPDATE_SELECTED_COUNT",
    count: number,
}

export const updateSelectedCount = actionCreator<{count: number}>("UPDATE_SELECTED_COUNT")

export const updateTracks = actionCreator<{tracks: Track[]}>("UPDATE_TRACKS")

export const updateQuery = createAsync<{query: string}, {}>("UPDATE_QUERY", (query, dispatch) => {
    try {
        const tracks = window.seiri.queryTracks(query.query)
        // tslint:disable-next-line:no-console
        dispatch(updateTracks(tracks))
    } catch (e) {
        // tslint:disable-next-line:no-console
        console.log("invalid bang?", e)
    }
    return { type: "UPDATE_QUERY", query }
})

export const updateTracksTick = createAsync<{}, {}>("UPDATE_TRACKS_TICK", (query, dispatch, getState) => {
    const state = getState();
    try {
        const tracks = window.seiri.queryTracks(state.query)
        dispatch(updateTracks(tracks))
    } catch (err) {
        // tslint:disable-next-line:no-console
        console.log(err);
    }

    window.setTimeout(() => dispatch(updateTracksTick.action({})), 30000)
    return { type: "UPDATE_TRACKS_TICK" }
})
export type SeiriAction = UpdateQuery | UpdateTracks;