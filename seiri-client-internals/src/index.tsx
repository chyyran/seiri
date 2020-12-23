import ReactDOM from "react-dom";
import { Provider } from 'react-redux'
import thunk from "redux-thunk";
import App from "./App";
import "./index.css";
import State from "./State"

import { applyMiddleware, createStore } from "redux";
import { composeWithDevTools } from "redux-devtools-extension";
import { reducerWithInitialState } from "typescript-fsa-reducers";
import { updateQuery, updateSelectedCount, updateTracks, updateTracksTick } from "./actions";

const initialState: State = {
  count: 0,
  query: "",
  tracks: [],
};

const reducer = reducerWithInitialState(initialState)
.case(updateSelectedCount, (state, {count}) => ({
  ...state,
  count
}))
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
  (<Provider store={store}>
    <App />
  </Provider>),
  document.getElementById("root"));
