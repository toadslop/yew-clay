import React from "react";
import ReactDOM from "react-dom";
import ClayButton from "@clayui/button";
import { DisplayType } from "@clayui/button/lib/Button";

export function render_clay_button(
  node: Element,
  alert: boolean,
  borderless: boolean,
  block: boolean,
  displayType: DisplayType,
  monospaced: boolean,
  outline: boolean,
  small: boolean,
  children: React.ReactNode,
  onClick: () => void,
  miscAttrs: {}
) {
  const element = React.createElement(ClayButton, {
    alert,
    borderless,
    block,
    displayType,
    monospaced,
    outline,
    small,
    children,
    onClick,
    ...miscAttrs,
  });
  ReactDOM.render(element, node);
}
