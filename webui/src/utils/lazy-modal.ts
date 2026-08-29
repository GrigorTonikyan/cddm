import React from "react";

/**
 * Creates a React.lazy component from a dynamic module import with a named export.
 */
export function lazyModal<T extends React.ComponentType<any>>(
  loader: () => Promise<Record<string, any>>,
  exportName: string,
): React.LazyExoticComponent<T> {
  return React.lazy(() =>
    loader().then((mod) => {
      const comp = mod[exportName];
      if (!comp) {
        throw new Error(`Export '${exportName}' not found in module.`);
      }
      return { default: comp as T };
    }),
  );
}
