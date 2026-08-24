import { useContext } from "react";
import { Win2xManagerContext } from "../context/win2x-manager-context";
import { Win2xManagerContextValue } from "../core/types";
import { WIN2X_ERRORS } from "../constants/win2x-constants";

export function useWindowManager(): Win2xManagerContextValue {
  const context = useContext(Win2xManagerContext);
  if (!context) {
    throw new Error(WIN2X_ERRORS.PROVIDER_MISSING);
  }
  return context;
}
