import * as React from "react";
import { DebounceInput } from "react-debounce-input";
import { connect, Dispatch } from "react-redux";
import { updateQuery, updateTracksTick } from "./actions";
import Helper from "./BangHelper";
import ElectronWindow from "./ElectronWindow";
import State from "./State";
import TrackTable from "./TrackTable";
import { Track } from "./types";
import "./View.css";

declare var window: ElectronWindow;
interface ViewProps {
  tracks: Track[];
  query: string;
  count: number;
  dispatch?: Dispatch<any>;
}

interface ViewState {
  showBangs: boolean;
}
const mapStateToProps = (state: State): ViewProps => {
  return { tracks: state.tracks, query: state.query, count: state.count };
};

const mapDispatchToProps = (
  dispatch: Dispatch<any>,
  ownProps: ViewProps
): ViewProps => {
  return { ...ownProps, dispatch };
};

// tslint:disable:jsx-no-lambda
class View extends React.Component<ViewProps, ViewState> {
  private queryInput: HTMLInputElement | null;
  constructor(props: ViewProps) {
    super(props);
    window.setTimeout(() => {
      this.props.dispatch!(updateQuery.action({ query: "" }));
      this.props.dispatch!(updateTracksTick.action());
    }, 0);
    this.state = {
      showBangs: false,
    }
    window.addEventListener("keydown", event => {
      if (!(event.ctrlKey || event.altKey)) {
        this.queryInput!.focus();
        return false;
      } else {
        return true;
      }
    });
  }

  public componentWillReceiveProps(newProps: ViewProps) {
    if (newProps.query === "??bangs") {
      // tslint:disable-next-line:no-console
      console.log("bang query detcted.");
      this.setState({ showBangs: true })

    } else {
      this.setState({ showBangs: false })
    }
  }

  public render() {
    return (
      <div className={this.props.tracks.length === 0 ? "container no-overflow" : "container"}>
      
        <div className="main-bar">
        <button className="btn-quit" onClick={() => this.hide()}>
            &#xe711;
          </button>
          <DebounceInput
            placeholder={
              'Type to start searching. Type "??bangs" for bang reference. Ctrl+R to refresh.'
            }
            inputRef={input => {
              this.queryInput = input;
            }}
            className="bang-input"
            minLength={1}
            debounceTimeout={100}
            // tslint:disable-next-line:no-console
            onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
              // tslint:disable-next-line:no-console
              this.props.dispatch!(
                updateQuery.action({ query: e.target.value })
              );
            }}
          />
        <div className="tracks">{this.props.tracks.length + " Tracks " + this.props.count + " Selected"}</div>
        </div>
        <div className="main-bar-clear" />

        <div className="tracks-containers">
          <TrackTable
            hidden={this.state.showBangs}
            tracks={this.props.tracks}
            query={this.props.query}
            dispatch={this.props.dispatch!}
          />
          <Helper hidden={!this.state.showBangs} />
        </div>
      </div>
    );
  }
  private hide() {
    window.require<any>('electron').remote.getCurrentWindow().hide();
  }
}

export default connect<ViewProps>(mapStateToProps, mapDispatchToProps)(View);
