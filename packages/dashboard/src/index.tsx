import "../style.scss";
//import "bootstrap/dist/css/bootstrap.min.css";
import React, { StrictMode, Suspense } from "react";
import ReactDOM from "react-dom";
import App from "./components/App";
import createRouter from "./utils/router";
import { rootStore, Store } from "./stores";
import "./utils/i18n";

const router = createRouter({
  // "/": ({ id }) => {
  //   console.log("test", id);
  // },
  "/": () => rootStore.viewStore.openHomePage(),
  "/card-overview/\\?page=(\\d+)": () => {
    const urlParams = new URLSearchParams(window.location.search);

    rootStore.viewStore.openCardOverview(
      Number.parseInt(urlParams.get("page") as string)
    );
  },
});

router(window.location.pathname + window.location.search);

// reaction(
//   () => viewStore.currentUrl,
//   (path) => {
//     if (window.location.pathname !== path) {
//       window.history.pushState(null, "", path);
//     }
//   }
// );

window.addEventListener("locationchange", function () {
  console.log("location changed!");
});

window.onpopstate = function historyChange(ev: any) {
  console.log("thetype", ev.type);
  if (ev.type === "popstate")
    router(window.location.pathname + window.location.search);
};

ReactDOM.render(
  <StrictMode>
    <Suspense
      fallback={
        <div>
          <h1>loading translatin</h1>
        </div>
      }
    >
      <Store.Provider value={rootStore}>
        <App />
      </Store.Provider>
    </Suspense>
  </StrictMode>,
  document.getElementById("root")
);
