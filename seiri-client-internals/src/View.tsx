import * as React from "react";
import { DebounceInput } from "react-debounce-input";
import { connect, Dispatch } from "react-redux";
import { updateQuery, updateTracksTick } from "./actions";
import State from "./State";
import TrackTable from "./TrackTable";
import { Track } from "./types";
import "./View.css";

interface ViewProps {
  tracks: Track[];
  query: string;
  dispatch?: Dispatch<any>;
}

const mapStateToProps = (state: State): ViewProps => {
  return { tracks: state.tracks, query: state.query };
};

const mapDispatchToProps = (
  dispatch: Dispatch<any>,
  ownProps: ViewProps
): ViewProps => {
  return { ...ownProps, dispatch };
};

// tslint:disable:jsx-no-lambda
class View extends React.Component<ViewProps> {
  private queryInput: HTMLInputElement | null;
  constructor(props: ViewProps) {
    super(props);
    window.setTimeout(() => {
      this.props.dispatch!(updateQuery.action({ query: "" }));
      this.props.dispatch!(updateTracksTick.action());
    }, 0);
    window.addEventListener('keydown', () => this.queryInput!.focus())
  }

  public render() {
    return (
      <div className="container">
        <div className="tracks-containers">
          <TrackTable tracks={this.props.tracks} />
        </div>
        <div className="main-bar">
          <DebounceInput
            inputRef={(input) => { this.queryInput = input; }}
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
          <button className="btn-quit" onClick={() => close()}>
            &#xe711;
          </button>
        </div>
      </div>
    );
  }
}

export default connect<ViewProps>(mapStateToProps, mapDispatchToProps)(View);
