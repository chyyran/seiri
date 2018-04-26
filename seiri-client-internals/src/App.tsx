import * as React from 'react';
import './App.css';
import SeiriProvider, {SeiriContext} from "./SeiriContext";

class App extends React.Component {
  public render() {
    return (
      <SeiriProvider>
        <SeiriContext.Consumer>
          {val => <div>Render OK</div>}
        </SeiriContext.Consumer>
      </SeiriProvider>
    );
  }
}

export default App;
