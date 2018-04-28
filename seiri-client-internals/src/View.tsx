import * as React from "react";
import { DebounceInput } from "react-debounce-input";
import { connect, Dispatch } from 'react-redux';
import { updateQuery, updateTracksTick } from "./actions";
import State from "./State";
import TrackTable from "./TrackTable";
import { Track } from "./types";
import "./View.css"

interface ViewProps {
    tracks: Track[],
    query: string,
    dispatch?: Dispatch<any>
}

const mapStateToProps = (state: State) : ViewProps => {
    return { tracks: state.tracks, query: state.query }
}

const mapDispatchToProps = (dispatch: Dispatch<any>, ownProps: ViewProps) : ViewProps => {
  return { ...ownProps, dispatch}
}

// tslint:disable:jsx-no-lambda
class View extends React.Component<ViewProps> {
  constructor(props: ViewProps) {
    super(props)
    window.setTimeout(() => {
      this.props.dispatch!(updateQuery.action({query: ""}))
      this.props.dispatch!(updateTracksTick.action())
    }, 0)
  }

  public render() {
    return (
      <div className="container">
        <div className="tracks-containers">
          <TrackTable tracks={this.props.tracks}/>
        </div>
        <DebounceInput
          className="bang-input"
          minLength={1}
          debounceTimeout={100}
          // tslint:disable-next-line:no-console
          onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
            // tslint:disable-next-line:no-console
            this.props.dispatch!(updateQuery.action({query: e.target.value}))
          }}
        />
      </div>
    );
  }

  // private launch(filename: string) {
  //   // tslint:disable-next-line:no-console
  //   const dirname = window.require("path").dirname(filename)
  //   // tslint:disable-next-line:no-console
  //   console.log(dirname)
  //   window.require("child_process").execFile(dirname)
  // }
}

export default connect<ViewProps>(mapStateToProps, mapDispatchToProps)(View);