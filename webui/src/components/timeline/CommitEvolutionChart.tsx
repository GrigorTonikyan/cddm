import React from "react";
import type { TimelineSnapshot, TimelineTrend } from "../../types/cddm-types";

export interface CommitEvolutionChartProps {
  snapshots: TimelineSnapshot[];
  timelineData?: TimelineTrend | null;
  hoveredSnapshot: TimelineSnapshot | null;
  setHoveredSnapshot: (s: TimelineSnapshot | null) => void;
}

export const CommitEvolutionChart: React.FC<CommitEvolutionChartProps> = ({
  snapshots,
  hoveredSnapshot,
  setHoveredSnapshot,
}) => {
  const svgWidth = 720;
  const svgHeight = 220;
  const padLeft = 45;
  const padRight = 30;
  const padTop = 25;
  const padBottom = 35;
  const chartW = svgWidth - padLeft - padRight;
  const chartH = svgHeight - padTop - padBottom;

  const getX = (index: number) => {
    if (snapshots.length <= 1) return padLeft + chartW / 2;
    return padLeft + (index / (snapshots.length - 1)) * chartW;
  };

  const getYScore = (score: number) => {
    const clamped = Math.max(0, Math.min(100, score));
    return padTop + chartH - (clamped / 100) * chartH;
  };

  const getYDuplication = (dup: number) => {
    const maxDupScale = 50;
    const clamped = Math.max(0, Math.min(maxDupScale, dup));
    return padTop + chartH - (clamped / maxDupScale) * chartH;
  };

  const dryPoints = snapshots
    .map((s, idx) => `${getX(idx)},${getYScore(s.dry_health_score)}`)
    .join(" ");
  const dupPoints = snapshots
    .map((s, idx) => `${getX(idx)},${getYDuplication(s.duplication_percentage)}`)
    .join(" ");

  return (
    <div className="bg-slate-900/80 border border-slate-800/80 rounded-xl p-4 shadow-lg">
      <div className="flex items-center justify-between mb-3 text-xs">
        <span className="font-bold text-slate-300 uppercase tracking-wider">
          DRY Health & Duplication Trajectory
        </span>
        {hoveredSnapshot ? (
          <div className="flex items-center gap-3 font-mono text-[11px] bg-slate-950 px-2.5 py-1 rounded-lg border border-slate-800">
            <span className="text-indigo-300 font-bold">{hoveredSnapshot.short_hash}</span>
            <span className="text-emerald-400">
              DRY: {hoveredSnapshot.dry_health_score.toFixed(1)}
            </span>
            <span className="text-rose-400">
              Dup: {hoveredSnapshot.duplication_percentage.toFixed(1)}%
            </span>
            <span className="text-slate-400">{hoveredSnapshot.formatted_date}</span>
          </div>
        ) : (
          <span className="text-slate-500 text-[11px]">
            Hover over data points to inspect snapshot
          </span>
        )}
      </div>

      <div className="w-full overflow-x-auto">
        <svg
          viewBox={`0 0 ${svgWidth} ${svgHeight}`}
          className="w-full h-48 select-none overflow-visible"
        >
          <line
            x1={padLeft}
            y1={padTop}
            x2={padLeft + chartW}
            y2={padTop}
            stroke="#334155"
            strokeDasharray="3 3"
            strokeOpacity="0.4"
          />
          <line
            x1={padLeft}
            y1={padTop + chartH / 2}
            x2={padLeft + chartW}
            y2={padTop + chartH / 2}
            stroke="#334155"
            strokeDasharray="3 3"
            strokeOpacity="0.4"
          />
          <line
            x1={padLeft}
            y1={padTop + chartH}
            x2={padLeft + chartW}
            y2={padTop + chartH}
            stroke="#475569"
            strokeWidth="1.5"
          />

          <text
            x={padLeft - 8}
            y={padTop + 4}
            fill="#64748b"
            fontSize="9"
            textAnchor="end"
            fontFamily="monospace"
          >
            100
          </text>
          <text
            x={padLeft - 8}
            y={padTop + chartH / 2 + 3}
            fill="#64748b"
            fontSize="9"
            textAnchor="end"
            fontFamily="monospace"
          >
            50
          </text>
          <text
            x={padLeft - 8}
            y={padTop + chartH + 3}
            fill="#64748b"
            fontSize="9"
            textAnchor="end"
            fontFamily="monospace"
          >
            0
          </text>

          {snapshots.length > 1 && (
            <>
              <polyline
                fill="none"
                stroke="#34d399"
                strokeWidth="2.5"
                points={dryPoints}
                strokeLinecap="round"
                strokeLinejoin="round"
              />
              <polyline
                fill="none"
                stroke="#f43f5e"
                strokeWidth="2"
                strokeDasharray="4 4"
                points={dupPoints}
                strokeLinecap="round"
                strokeLinejoin="round"
              />
            </>
          )}

          {snapshots.map((s, idx) => {
            const cx = getX(idx);
            const cyScore = getYScore(s.dry_health_score);
            const isHovered = hoveredSnapshot?.commit_hash === s.commit_hash;
            return (
              <g
                key={s.commit_hash}
                className="cursor-pointer"
                onMouseEnter={() => setHoveredSnapshot(s)}
                onMouseLeave={() => setHoveredSnapshot(null)}
              >
                <circle
                  cx={cx}
                  cy={cyScore}
                  r={isHovered ? 6 : 4}
                  fill={isHovered ? "#10b981" : "#059669"}
                  stroke="#0f172a"
                  strokeWidth="2"
                />
                <text
                  x={cx}
                  y={padTop + chartH + 18}
                  fill={isHovered ? "#e2e8f0" : "#64748b"}
                  fontSize="9"
                  textAnchor="middle"
                  fontFamily="monospace"
                >
                  {s.short_hash}
                </text>
              </g>
            );
          })}
        </svg>
      </div>
    </div>
  );
};
