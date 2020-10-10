//import history from "history/browser";
import "bootstrap/dist/css/bootstrap.min.css";
import React from "react";
import ReactDOM from "react-dom";
import App from "./App";
// const t = (t: number) => {};

// t("");
// console.log("test");

// // Listen for changes to the current location.
// const unlisten = history.listen(({ location, action }) => {
//   console.log(action, location.pathname, location.state);
// });

// unlisten();

ReactDOM.render(<App />, document.getElementById("root"));
