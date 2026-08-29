import React, { useRef, useState } from "react";
import { Plus, X, ChevronLeft, ChevronRight } from "lucide-react";
import { WIN2X_DATA_ATTRS } from "../../constants/win2x-constants";
import { TabItemData } from "../../core/types";
import styles from "./tab-bar.module.css";

export interface TabBarProps {
  tabs: TabItemData[];
  activeTabId: string | null;
  onTabSelect: (id: string) => void;
  onTabClose?: (id: string) => void;
  onTabAdd?: () => void;
  className?: string;
}

export const TabBar: React.FC<TabBarProps> = ({
  tabs,
  activeTabId,
  onTabSelect,
  onTabClose,
  onTabAdd,
  className = "",
}) => {
  const scrollRef = useRef<HTMLDivElement>(null);
  const [canScrollLeft, setCanScrollLeft] = useState(false);
  const [canScrollRight, setCanScrollRight] = useState(false);

  const checkScroll = () => {
    if (scrollRef.current) {
      const { scrollLeft, scrollWidth, clientWidth } = scrollRef.current;
      setCanScrollLeft(scrollLeft > 0);
      setCanScrollRight(Math.ceil(scrollLeft + clientWidth) < scrollWidth);
    }
  };

  React.useEffect(() => {
    checkScroll();
    window.addEventListener("resize", checkScroll);
    return () => window.removeEventListener("resize", checkScroll);
  }, [tabs]);

  const scrollBy = (offset: number) => {
    if (scrollRef.current) {
      scrollRef.current.scrollBy({ left: offset, behavior: "smooth" });
      setTimeout(checkScroll, 300); // Check again after smooth scroll
    }
  };

  return (
    <div className={`${styles.tabBar} ${className}`} {...{ [WIN2X_DATA_ATTRS.TAB_BAR]: true }}>
      {canScrollLeft && (
        <button
          type="button"
          className={styles.scrollBtn}
          onClick={() => scrollBy(-150)}
          aria-label="Scroll left"
        >
          <ChevronLeft size={16} />
        </button>
      )}

      <div ref={scrollRef} className={styles.scrollContainer} onScroll={checkScroll}>
        {tabs.map((tab) => {
          const isActive = tab.id === activeTabId;
          return (
            <button
              key={tab.id}
              type="button"
              className={`${styles.tab} ${isActive ? styles.active : ""}`}
              onClick={() => onTabSelect(tab.id)}
              disabled={tab.disabled}
              {...{ [WIN2X_DATA_ATTRS.TAB_ITEM]: true }}
            >
              {tab.icon && <div className={styles.tabIcon}>{tab.icon}</div>}
              <span className={styles.tabTitle}>{tab.title}</span>

              {tab.badgeCount !== undefined && (
                <span
                  className={`${styles.tabBadge} ${tab.badgeVariant ? styles[`badge-${tab.badgeVariant}`] : ""}`}
                >
                  {tab.badgeCount}
                </span>
              )}

              {tab.closable !== false && onTabClose && (
                <div
                  role="button"
                  tabIndex={0}
                  className={styles.tabClose}
                  onClick={(e) => {
                    e.stopPropagation();
                    onTabClose(tab.id);
                  }}
                  onKeyDown={(e) => {
                    if (e.key === "Enter" || e.key === " ") {
                      e.stopPropagation();
                      e.preventDefault();
                      onTabClose(tab.id);
                    }
                  }}
                >
                  <X size={12} />
                </div>
              )}
            </button>
          );
        })}
      </div>

      {canScrollRight && (
        <button
          type="button"
          className={styles.scrollBtn}
          onClick={() => scrollBy(150)}
          aria-label="Scroll right"
        >
          <ChevronRight size={16} />
        </button>
      )}

      {onTabAdd && (
        <button type="button" className={styles.addBtn} onClick={onTabAdd} aria-label="Add new tab">
          <Plus size={16} />
        </button>
      )}
    </div>
  );
};
