import * as React from "react";
import { DebounceInput } from "react-debounce-input";
import "./App.css";
import SeiriProvider, { SeiriContext } from "./SeiriContext";

// tslint:disable:jsx-no-lambda
class App extends React.Component {
  public render() {
    return (
      <SeiriProvider>
        <SeiriContext.Consumer>
          {val => (
            <div>
              <DebounceInput
                minLength={1}
                debounceTimeout={300}
                onChange={(e: React.ChangeEvent<HTMLInputElement>) => val.updateQuery!(e.target.value)}
              />
              {val.latestQueryString === ""
                ? val.allTracks.map(e => <div key={e.filePath}>{e.title}</div>)
                : val.latestQueryTracks.map(e => (
                    <div key={e.filePath}>{e.title}</div>
                  ))}
            </div>
          )}
        </SeiriContext.Consumer>
      </SeiriProvider>
    );
  }
}

export default App;
