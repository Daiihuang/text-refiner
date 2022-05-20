import {
  Modal,
  ModalHeader,
  ModalContent,
  ModalFooter,
  Button,
  ModalBody,
  Select,
  SelectItem,
  Input,
} from "@nextui-org/react";
import { Store } from "@tauri-apps/plugin-store";
import { useContext } from "react";
import { SettingContext } from "../providers/settings";
import { providerMap } from "../types/settings";

const store_api_key = async (api_key: string) => {
  const store = new Store("store.bin");
  store.set("OPENAI_API_KEY", api_key);
  await store.save();
  console.log("Stored API key");
  let duma = await store.get("OPENAI_API_KEY");
  console.log("Stored key: ", duma);
};

const Settings = ({
  isOpen,
  onOpen,
  onOpenChange,
}: {
 