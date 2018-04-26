import { ITrack } from "./types";

export default interface IState {
    tracks: ITrack[],
    query: string
}