import { Select, SelectItem } from "@nextui-org/select";
import { CiSettings } from "react-icons/ci";
import { Settings } from "./settings";
import React from "react";
import { useContext } from "react";
import { SettingContext } from "../providers/settings";
import { FaWandMagicSparkles } from "react-icons/fa6";

const BottomBar = () => {
  const [openSettings, setOpenSettings] = React.useState(false);
  const settingContext = useContext(SettingContext);

  return (
    <div className="fixed bottom-0 left-0 flex h-12 w-full items-center justify-between rounded-b-extra bg-gray-200 pl-3 pr-3">
      <div className="flex space-x-3 w-full ite