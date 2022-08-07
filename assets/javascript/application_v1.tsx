import React from "react";
import { importComponents, mountComponents } from "lib/ujs";
import "../css/application_v1.css";

importComponents();

window.addEventListener("load", (_e) => mountComponents());
