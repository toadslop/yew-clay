import React from 'react';
import { DisplayType } from '@clayui/button/lib/Button';

declare function render_clay_button(node: Element, alert: boolean, borderless: boolean, block: boolean, displayType: DisplayType, monospaced: boolean, outline: boolean, small: boolean, children: React.ReactNode, onClick: () => void, miscAttrs: {}): void;
declare function render_clay_button_group(node: Element, spaced: boolean, vertical: boolean, role: string, miscAttrs: {}): void;

export { render_clay_button, render_clay_button_group };
