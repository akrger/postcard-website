import { createContext } from "react";
import CardStore from "./CardStore";
import ViewStore from "./ViewStore";

const viewStore = new ViewStore();
const cardStore = new CardStore();

export const rootStore = { viewStore, cardStore };

export const Store = createContext<typeof rootStore>(rootStore);
