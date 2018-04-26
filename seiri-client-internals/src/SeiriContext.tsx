import * as diff from "deep-diff";
import * as React from "react";

enum TrackFileType {
  FLAC,
  FLAC4,
  FLAC8,
  FLAC16,
  FLAC24,
  FLAC32,
  ALAC,
  MP3_CBR,
  MP3_VBR,
  AAC,
  VORBIS,
  OPUS,
  WAVPACK,
  MONKEYS_AUDIO,
  UNKNOWN
}

interface ITrack {
  filePath: string;
  title: string;
  artist: string;
  albumArtists: string[];
  album: string;
  year: number;
  tracknumber: number;
  musicbrainzTrackId: string;
  hasFrontCover: boolean;
  frontCoverHeight: number;
  frontCoverWidth: number;
  bitrate: number;
  sampleRate: number;
  source: string;
  discnumber: number;
  duration: number;
  fileType: TrackFileType;
}

interface ITrackCache {
  allTracks: ITrack[];
  latestQueryString: string;
  latestQueryTracks: ITrack[];
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
      latestQueryTracks: []
    };
    navigator.serviceWorker.addEventListener("message", event => {
      // tslint:disable-next-line:no-console
      console.log("Received message from service worker.");
      this.updateFromServiceWorker(event.data);
    });
  }

  public updateFromServiceWorker(data: IWorkerResponse) {
    // tslint:disable-next-line:no-console
    console.log(data);
    if (data.type === "diff-all") {
      // tslint:disable-next-line:no-console
      console.log("Received new diff-all.");
      const allTracks = this.state.allTracks;
      for (const trackDiff of data.payload as deepDiff.IDiff[]) {
        diff.applyChange(allTracks, {}, trackDiff);
      }
      this.setState({
        allTracks
      });
    }

    if (data.type === "diff-query") {
        // tslint:disable-next-line:no-console
        console.log("Received new diff-query.");
        const latestQueryTracks = this.state.latestQueryTracks;
        for (const trackDiff of data.payload as deepDiff.IDiff[]) {
          diff.applyChange(latestQueryTracks, {}, trackDiff);
        }
        this.setState({
          latestQueryTracks
        });
      }

    if (data.type === "state-all") {
      // tslint:disable-next-line:no-console
      console.log("Received new State.");
      this.setState({
        allTracks: data.payload as ITrack[]
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
