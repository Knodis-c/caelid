import React from "react";
import { importComponents, mountComponents } from "lib/ujs";

importComponents();

window.addEventListener("load", (_e) => mountComponents());
