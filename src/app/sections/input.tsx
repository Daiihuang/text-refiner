"use client";

import React from "react";
import { Textarea } from "@nextui-org/react";

const TextInput = ({
  inputText,
  changeText,
}: {
  inputText?: string;
  changeText: (text: string) => void;
}) => {
  return (
    <Textarea
      id="input-text"
      variant="bordered"
  