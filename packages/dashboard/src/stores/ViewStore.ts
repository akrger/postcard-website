import {
  action,
  //action,
  makeAutoObservable,
  // makeObservable, observable
} from "mobx";

type Page = "home" | "card-overview" | "notfound";
type Route = "/" | "/card-overview" | "/404";

export default class ViewStore {
  page: Page = "notfound";
  loading = false;
  currentPageIndex = 1;

  constructor() {
    makeAutoObservable(this);
  }

  get currentUrl(): Route {
    console.log("currenturl", this.page);
    switch (this.page) {
      case "home":
        return "/";
      case "card-overview":
        return "/card-overview";

      case "notfound":
        return "/404";
    }
  }

  openCardOverview(page: number): void {
    console.log("opencardoverview", page);
    this.loading = true;
    this.page = "card-overview";

    this.currentPageIndex = page;

    //this.loading = false;

    setTimeout(
      () =>
        action(() => {
          this.loading = false;
        }),
      1000
    );
    setTimeout(() => {
      this.loading = false;
    }, 1000);

    fetch("/api/user/").then((e) => {
      //console.log("uber", e);
      e.json().then((e) => console.log(e));
      setTimeout(
        action(() => {
          this.loading = false;
        }),
        1000
      );
    });
  }

  openHomePage(): void {
    this.page = "home";
  }
}
