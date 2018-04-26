import * as React from "react";
import * as ReactDOM from "react-dom";
// import registerServiceWorker from './registerServiceWorker';
import { Provider } from 'react-redux'
import thunk from "redux-thunk"; // no changes here 😀
import App from "./App";
import "./index.css";
import State from "./State"

import { applyMiddleware, createStore } from "redux";
import { composeWithDevTools } from "redux-devtools-extension";
import { SeiriAction } from "./actions";


const reducer = (state: State, action: SeiriAction): State => {
  switch (action.type) {
    case "UPDATE_QUERY":
      return { ...state, query: action.query };
    case "UPDATE_TRACKS":
      return { ...state, tracks: action.tracks };
    default:
      return state;
  }
}


const store = createStore(
  reducer,
  composeWithDevTools(
    applyMiddleware(thunk)
    // other store enhancers if any
  )
);

ReactDOM.render(
  <Provider store={store}>
    <App />
  </Provider>, 
document.getElementById("root") as HTMLElement);
// registerServiceWorker();
