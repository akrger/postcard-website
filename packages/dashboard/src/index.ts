import history from "history/browser";
import React from "react";

const t = (t: number) => {};

t("");
console.log("test");

// Listen for changes to the current location.
const unlisten = history.listen(({ location, action }) => {
  console.log(action, location.pathname, location.state);
});

unlisten();
