import { match } from "path-to-regexp";

type RouteParams = Record<string, number>;
type RouteFunction = (routeParams: RouteParams) => void;

type Routes = {
  [route: string]: RouteFunction;
};

const __match = match("/user/", { decode: decodeURIComponent });

console.log(__match("/user/")); //=> { path: '/user/123', index: 0, params: { id: '123' } }

function createRouter(routes: Routes): (path: string) => boolean {
  const matchers = Object.keys(routes).map((path) => [
    match(path, { decode: decodeURIComponent }),
    routes[path] as () => void,
  ]);
  return (path: string) => {
    return matchers.some(([matcher, func]) => {
      const result = matcher(path);
      console.log("match", result, path);
      if (result === false) {
        return false;
      }
      if (result) {
        (func as RouteFunction)(result.params as RouteParams);
      }
      return true;
    });
  };
}

export default createRouter;
