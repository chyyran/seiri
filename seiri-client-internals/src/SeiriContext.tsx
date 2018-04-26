import * as diff from "deep-diff";
import * as React from "react";
import { ITrack } from "./types";

interface ITrackCache {
  allTracks: ITrack[];
  latestQueryString: string;
  latestQueryTracks: ITrack[];
  updateQuery?: (query: string) => void;
}

interface IWorkerResponse {
  type: string;
  payload: ITrack[] | deepDiff.IDiff[];
}

export const SeiriContext = React.createContext<ITrackCache>({
  allTracks: [],
  latestQueryString: "",
  latestQueryTracks: []
});

class SeiriProvider extends React.Component<{}, ITrackCache> {
  public constructor() {
    super({});
    this.state = {
      allTracks: [],
      latestQueryString: "",
      latestQueryTracks: [],
      updateQuery: this.updateQuery.bind(this)
    };
    navigator.serviceWorker.addEventListener("message", event => {
      // tslint:disable-next-line:no-console
      console.log("Received message from service worker.");
      this.updateFromServiceWorker(event.data);
    });
  }

  public updateQuery(query: string) {
    this.setState({ latestQueryString: query });
    if (query !== "") {
      navigator.serviceWorker.controller!.postMessage({ query });
    } else {
        this.setState({latestQueryTracks: []})
    }
  }

  public updateFromServiceWorker(data: IWorkerResponse) {
    // tslint:disable-next-line:no-console
    console.log(data);
    if (data.type === "diff-all") {
      // tslint:disable-next-line:no-console
      console.log("Received new diff-all.");
      const allTracks = this.state.allTracks;
      window.setTimeout(() => {
        for (const trackDiff of data.payload as deepDiff.IDiff[]) {
          diff.applyChange(allTracks, {}, trackDiff);
        }
        this.setState({
          allTracks
        });
      }, 0);
    }

    if (data.type === "diff-query") {
      // tslint:disable-next-line:no-console
      console.log("Received new diff-query.");
      const latestQueryTracks = this.state.latestQueryTracks;
      window.setTimeout(() => {
        for (const trackDiff of data.payload as deepDiff.IDiff[]) {
          diff.applyChange(latestQueryTracks, {}, trackDiff);
        }
        this.setState({
          latestQueryTracks
        });
      }, 0);
    }

    if (data.type === "state-all") {
      // tslint:disable-next-line:no-console
      console.log("Received new State.");
      this.setState({
        allTracks: data.payload as ITrack[]
      });
    }

    if (data.type === "state-query") {
      // tslint:disable-next-line:no-console
      console.log("Received new State.");
      this.setState({
        latestQueryTracks: data.payload as ITrack[]
      });
    }
  }

  public componentWillMount() {
    navigator.serviceWorker.ready.then(value => {
      // tslint:disable-next-line:no-console
      console.log("Service Worker Ready!");
      value.active!.postMessage({ query: "" });
    });
  }

  public render() {
    return (
      <SeiriContext.Provider value={this.state}>
        {this.props.children}
      </SeiriContext.Provider>
    );
  }
}

export default SeiriProvider;
