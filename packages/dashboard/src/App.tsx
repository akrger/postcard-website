import React, { useEffect } from "react";

import history from "history/browser";
import { match } from "path-to-regexp";
// Alternatively, if you're using hash history import
// the hash history singleton instance.
// import history from 'history/hash';

// Get the current location.

// Listen for changes to the current location.

//const router = createRouter({
// "/:userid": ({ userid }) => {
//   console.log(userid);
//   console.log("worked");
// },
//});

const App = () => {
  useEffect(() => {
    history.listen(({ location, action }) => {
      // console.log("ozy");
      console.log(action, location.pathname, location.state);
    });
    fetch("/user/").then(async (e) => console.log(await e.json()));
    // return () => unlisten();
  }, []);
  console.log("tet");
  return (
    <div>
      <button onClick={() => history.push("/abc")}></button>
    </div>
  );
};

type RouteParams = Record<string, number>;
type RouteFunction = (routeParams: RouteParams) => void;

type Routes = {
  [route: string]: RouteFunction;
};

const t = (routes: Routes) => {
  const matchers = Object.keys(routes).map((path) => [
    match(path, { decode: decodeURIComponent }),
    routes[path] as () => void,
  ]);
  return (path: string) => {
    return matchers.some(([matcher, func]) => {
      const result = matcher(path);
      if (result === false) {
        return false;
      }
      if (result) {
        (func as RouteFunction)(result.params as RouteParams);
      }
      return true;
    });
  };
};

const router = t({
  "/:id": ({ id }) => {
    console.log("test", id);
  },
  b: () => {
    console.log();
  },
});

router(window.location.pathname);
export default App;
