import * as React from "react";
import { DebounceInput } from "react-debounce-input";
import { connect, Dispatch } from 'react-redux';
import { updateQuery, updateTracksTick } from "./actions";
import State from "./State";
import { Track } from "./types";

interface ListProps {
    tracks: Track[],
    query: string,
    dispatch?: Dispatch<any>
}

const mapStateToProps = (state: State) : ListProps => {
    return { tracks: state.tracks, query: state.query }
}

const mapDispatchToProps = (dispatch: Dispatch<any>, ownProps: ListProps) : ListProps => {
  return { ...ownProps, dispatch}
}

// tslint:disable:jsx-no-lambda
class List extends React.Component<ListProps> {
  constructor(props: ListProps) {
    super(props)
    window.setTimeout(() => {
      this.props.dispatch!(updateQuery.action({query: ""}))
      this.props.dispatch!(updateTracksTick.action())
    }, 0)
  }
  public render() {
    return (
      <div>
        <DebounceInput
          style={{position: 'fixed'}}
          minLength={1}
          debounceTimeout={100}
          // tslint:disable-next-line:no-console
          onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
            // tslint:disable-next-line:no-console
            this.props.dispatch!(updateQuery.action({query: e.target.value}))
          }}
        />
        {
          this.props.tracks.map(t => <div key={t.filePath}>{t.title}</div>)
        }
      </div>
    );
  }
}

export default connect<ListProps>(mapStateToProps, mapDispatchToProps)(List);