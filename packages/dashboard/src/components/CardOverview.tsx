import { observer } from "mobx-react-lite";
import React, { ReactElement, useContext, useState } from "react";
import { useTranslation } from "react-i18next";
import { Store } from "../stores";
import { List } from "@postcard-website/common";

type T = { test: number };

function CardOverView(): ReactElement {
  const { viewStore } = useContext(Store);
  const { t } = useTranslation();
  const [index, setIndex] = useState(viewStore.currentPageIndex);

  const pagination = 3;

  const data = [{ test: 1 }, { test: 2 }, { test: 3 }, { test: 4 }];

  const urlParams = new URLSearchParams(window.location.search);

  // TODO: do this in the router config
  if (urlParams.has("test")) {
    //console.log("yeah");
    //setIndex(Number.parseInt(urlParams.get("test")));
  }
  return (
    <div>
      <h1>{t("card-overview")}</h1>
      <List
        pagination={pagination}
        renderItem={(item) => <h1 key={item.test}>{item.test}</h1>}
        currentIndex={index}
        data={data}
        onPressPaginationIndex={(index) => {
          //viewStore.page = "card-overview";
          setIndex(index);
          window.history.pushState(
            null,
            "",
            viewStore.currentUrl + `/?page=${index}`
          );
          //viewStore.openCardOverview();
        }}
      />
    </div>
  );
}

export default observer(CardOverView);
