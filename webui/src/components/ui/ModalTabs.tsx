import React from "react";

export interface ModalTabItem<T extends string> {
  id: T;
  label: string;
  icon?: React.ReactNode;
  count?: number;
}

export interface ModalTabsProps<T extends string> {
  tabs: ModalTabItem<T>[];
  activeTab: T;
  onTabChange: (id: T) => void;
  activeColorClass?: string;
  className?: string;
}

export function ModalTabs<T extends string>({
  tabs,
  activeTab,
  onTabChange,
  activeColorClass = "bg-indigo-600 text-white",
  className = "flex items-center gap-2",
}: ModalTabsProps<T>) {
  return (
    <div className={className}>
      {tabs.map((t) => {
        const isActive = activeTab === t.id;
        return (
          <button
            key={t.id}
            type="button"
            onClick={() => onTabChange(t.id)}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-all cursor-pointer ${
              isActive
                ? activeColorClass
                : "text-slate-400 hover:text-slate-200 hover:bg-slate-800/50"
            }`}
          >
            {t.icon}
            <span>
              {t.label}
              {t.count !== undefined ? ` (${t.count})` : ""}
            </span>
          </button>
        );
      })}
    </div>
  );
}
