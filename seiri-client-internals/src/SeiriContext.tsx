import * as React from "react";
import { ITrack } from "./types";

// tslint:disable-next-line:interface-name
interface ElectronWindow extends Window {
  require: any
}

declare var window: ElectronWindow;

interface ITrackCache {
  tracks: ITrack[];
  updateQuery?: (query: string) => void;
}

export const SeiriContext = React.createContext<ITrackCache>({
  tracks: []
});

class SeiriProvider extends React.Component<{}, ITrackCache> {
  private seiriNeon: (query: string) => ITrackCache;
  public constructor(props: {}) {
    super(props);
    this.seiriNeon = window.require("seiri-neon")
    this.state = {
      tracks: [],
      updateQuery: this.updateQuery.bind(this)
    };
    window.setInterval(() => this.state.updateQuery!(""), 5000);
  }

  public updateQuery(query: string) {
    const tracks = this.seiriNeon(query).tracks
    this.setState({ tracks });
  }

  public componentWillMount() {
    this.updateQuery("")
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
