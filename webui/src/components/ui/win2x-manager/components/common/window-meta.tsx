import React from "react";
import type { WindowRegistration } from "../../core/types";

export interface WindowMetaProps {
  win: Pick<WindowRegistration, "title"> &
    Partial<Pick<WindowRegistration, "subtitle" | "icon" | "badge">>;
  iconWrapperClass?: string;
  infoClass?: string;
  titleClass?: string;
  subtitleClass?: string;
  badgeClass?: string;
}

export const WindowMetaDisplay: React.FC<WindowMetaProps> = ({
  win,
  iconWrapperClass,
  infoClass,
  titleClass,
  subtitleClass,
  badgeClass,
}) => {
  return (
    <>
      {win.icon && <div className={iconWrapperClass}>{win.icon}</div>}
      <div className={infoClass}>
        <span className={titleClass}>{win.title}</span>
        {win.subtitle && <span className={subtitleClass}>{win.subtitle}</span>}
      </div>
      {win.badge && <span className={badgeClass}>{win.badge}</span>}
    </>
  );
};
