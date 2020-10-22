import { makeAutoObservable } from "mobx";

//type Page = "home" | "about" | "notfound";
//type Route = "/" | "/about" | "/404";

export default class CardStore {
  cool = "notfound";

  constructor() {
    makeAutoObservable(this);
  }
}
