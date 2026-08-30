import { ClonePair, TreemapNode, TreemapRect } from "../types/cddm-types";

interface HierarchyRawNode {
  name: string;
  path: string;
  tokens: number;
  clones: number;
  children: Map<string, HierarchyRawNode>;
}

/**
 * Builds a hierarchical directory tree from clone pairs and file paths.
 */
export const buildTreemapHierarchy = (clonePairs: ClonePair[]): TreemapNode => {
  const root: HierarchyRawNode = {
    name: "root",
    path: "",
    tokens: 0,
    clones: 0,
    children: new Map(),
  };

  const safePairs = Array.isArray(clonePairs) ? clonePairs : [];

  for (const pair of safePairs) {
    const recordFile = (filePath: string, tokens: number) => {
      const normalized = filePath.replace(/\\/g, "/").replace(/^\.\//, "");
      const segments = normalized.split("/");
      let curr = root;

      let currentPath = "";
      for (let i = 0; i < segments.length; i++) {
        const seg = segments[i];
        if (!seg) continue;
        currentPath = currentPath ? `${currentPath}/${seg}` : seg;

        if (!curr.children.has(seg)) {
          curr.children.set(seg, {
            name: seg,
            path: currentPath,
            tokens: 0,
            clones: 0,
            children: new Map(),
          });
        }
        curr = curr.children.get(seg)!;
        curr.tokens += tokens;
        curr.clones += 1;
      }
    };

    recordFile(pair.file_a, pair.token_count);
    recordFile(pair.file_b, pair.token_count);
  }

  const convertRawToTreemap = (raw: HierarchyRawNode): TreemapNode => {
    const childrenList = Array.from(raw.children.values()).map(convertRawToTreemap);
    const totalChildTokens = childrenList.reduce((sum, c) => sum + c.tokens, 0);
    const totalTokens = Math.max(raw.tokens, totalChildTokens, 1);
    const totalClones = Math.max(
      raw.clones,
      childrenList.reduce((sum, c) => sum + c.clones, 0),
    );

    const duplicationPercentage = Math.min(
      100,
      Math.max(5, (totalClones * 12) / Math.max(1, childrenList.length || 1)),
    );

    return {
      name: raw.name,
      path: raw.path,
      tokens: totalTokens,
      clones: totalClones,
      duplicationPercentage,
      children: childrenList.length > 0 ? childrenList : undefined,
    };
  };

  return convertRawToTreemap(root);
};

/**
 * Standard Squarified Treemap layout algorithm (Bruls et al.).
 */
export const computeSquarifiedLayout = (
  nodes: TreemapNode[],
  x: number,
  y: number,
  width: number,
  height: number,
): TreemapRect[] => {
  if (nodes.length === 0 || width <= 0 || height <= 0) return [];

  const totalValue = nodes.reduce((sum, n) => sum + n.tokens, 0);
  if (totalValue <= 0) return [];

  const rects: TreemapRect[] = [];
  const sorted = [...nodes].sort((a, b) => b.tokens - a.tokens);

  let remainingNodes = [...sorted];
  let curX = x;
  let curY = y;
  let curW = width;
  let curH = height;

  while (remainingNodes.length > 0) {
    const isHorizontal = curW >= curH;
    const remainingTotal = remainingNodes.reduce((sum, n) => sum + n.tokens, 0);

    const row: TreemapNode[] = [remainingNodes[0]!];
    remainingNodes = remainingNodes.slice(1);

    let rowTotal = row[0]!.tokens;

    const worst = (r: TreemapNode[]) => {
      const s = r.reduce((sum, item) => sum + item.tokens, 0);
      const rArea = (s / remainingTotal) * (curW * curH);
      const rowLen = isHorizontal ? rArea / curH : rArea / curW;
      if (rowLen <= 0) return Infinity;

      let maxAspect = 0;
      for (const item of r) {
        const itemArea = (item.tokens / remainingTotal) * (curW * curH);
        const itemLen = itemArea / rowLen;
        const aspect = Math.max(rowLen / itemLen, itemLen / rowLen);
        if (aspect > maxAspect) maxAspect = aspect;
      }
      return maxAspect;
    };

    while (remainingNodes.length > 0) {
      const next = remainingNodes[0]!;
      const newRow = [...row, next];
      if (worst(newRow) <= worst(row)) {
        row.push(next);
        rowTotal += next.tokens;
        remainingNodes = remainingNodes.slice(1);
      } else {
        break;
      }
    }

    const rowArea = (rowTotal / remainingTotal) * (curW * curH);
    if (isHorizontal) {
      const rowWidth = Math.max(1, rowArea / curH);
      let itemY = curY;
      for (const item of row) {
        const itemHeight = (item.tokens / rowTotal) * curH;
        rects.push({
          x: curX,
          y: itemY,
          width: rowWidth,
          height: itemHeight,
          node: item,
        });
        itemY += itemHeight;
      }
      curX += rowWidth;
      curW -= rowWidth;
    } else {
      const rowHeight = Math.max(1, rowArea / curW);
      let itemX = curX;
      for (const item of row) {
        const itemWidth = (item.tokens / rowTotal) * curW;
        rects.push({
          x: itemX,
          y: curY,
          width: itemWidth,
          height: rowHeight,
          node: item,
        });
        itemX += itemWidth;
      }
      curY += rowHeight;
      curH -= rowHeight;
    }
  }

  return rects;
};
