import React, {
  ReactElement,
  useContext,
  // useEffect
} from "react";
import { observer } from "mobx-react-lite";
//import history from "history/browser";
import ViewStore from "../stores/ViewStore";
import CardOverView from "./CardOverview";
import { Store } from "../stores";
import { Trans, useTranslation } from "react-i18next";
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

function App(): ReactElement {
  const { t } = useTranslation();

  const store = useContext(Store);
  // useEffect(() => {
  //   history.listen(({ location, action }) => {
  //     // console.log("ozy");
  //     console.log(action, location.pathname, location.state);
  //   });
  //  // fetch("/api/user/").then(async (e) => console.log(await e.json()));
  //   // return () => unlisten();
  // }, []);
  console.log(store.viewStore);
  if (store.viewStore.loading) {
    return (
      <div>
        <h1>loading</h1>
      </div>
    );
  }
  return <div>{renderPage(store.viewStore, t)}</div>;
}

const renderPage = (viewStore: ViewStore, t) => {
  console.log("thepage", viewStore.page);
  switch (viewStore.page) {
    case "home":
      return (
        <div>
          <h1>tttlnt: {t("welcome.text")}</h1>
          <Trans i18nKey="Welcome to React">Coo stuff</Trans>
          <h1>Home</h1>
          <a href="/card-overview/?page=1">card overview</a>
        </div>
      );
    case "card-overview":
      return <CardOverView />;
    case "notfound":
      return (
        <div>
          <h1>not found</h1>
        </div>
      );
  }
};

export default observer(App);
