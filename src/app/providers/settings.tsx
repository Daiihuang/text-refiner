import React from "react";
import { AppSettings, Prompts, Provider, providerMap } from "../types/settings";
const SettingContext = React.createContext({} as AppSettings);

const SettingProvider = ({ children }: {