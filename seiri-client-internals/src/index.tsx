import * as React from "react";
import * as ReactDOM from "react-dom";
// import registerServiceWorker from './registerServiceWorker';
import { Provider } from 'react-redux'
import thunk from "redux-thunk";
import App from "./App";
import "./index.css";
import State from "./State"

import { applyMiddleware, createStore } from "redux";
import { composeWithDevTools } from "redux-devtools-extension";
import { reducerWithInitialState } from 'typescript-fsa-reducers';
import { updateQuery, updateTracks, updateTracksTick } from "./actions";

const initialState: State = {
  query: "",
  tracks: []
};

const reducer = reducerWithInitialState(initialState)
.case(updateTracks, (state, {tracks}) => ({
  ...state,
  tracks
}))
.case(updateQuery.async.done, (state, {params}) => ({
  ...state,
  query: params.query
}))
.case(updateTracksTick.async.done, (state) => ({
  ...state,
}));;


const store = createStore(reducer,
    composeWithDevTools(applyMiddleware(thunk)),
);

ReactDOM.render(
  <Provider store={store}>
    <App />
  </Provider>, 
document.getElementById("root") as HTMLElement);
// registerServiceWorker();
