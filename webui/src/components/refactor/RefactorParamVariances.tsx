import React from "react";
import { CollapsibleCard, CodeBlock, BADGE_VARIANTS, CODE_BLOCK_VARIANTS } from "../ui";
import { AlertCircle } from "lucide-react";
import { RefactorSuggestion } from "../../types/cddm-types";

interface RefactorParamVariancesProps {
  pairSuggestion: RefactorSuggestion;
  filenameA: string;
  filenameB: string;
}

export const RefactorParamVariances: React.FC<RefactorParamVariancesProps> = ({
  pairSuggestion,
  filenameA,
  filenameB,
}) => {
  if (pairSuggestion.parameter_differences.length === 0) {
    return null;
  }

  return (
    <CollapsibleCard
      icon={<AlertCircle className="w-4 h-4" />}
      title="Parameter Variances"
      badgeCount={pairSuggestion.parameter_differences.length}
      badgeVariant={BADGE_VARIANTS.AMBER}
      defaultOpen={true}
    >
      <div className="space-y-3">
        {pairSuggestion.parameter_differences.map((diff, i) => (
          <div
            key={`param-diff-${diff.line_number_a}-${diff.line_number_b}-${i}`}
            className="grid grid-cols-1 md:grid-cols-2 gap-3"
          >
            <CodeBlock
              filename={filenameA}
              lineRange={`L${diff.line_number_a}`}
              code={diff.fragment_a_code}
              variant={CODE_BLOCK_VARIANTS.REMOVED}
              showCopy={true}
              emptyPlaceholder="<empty>"
            />
            <CodeBlock
              filename={filenameB}
              lineRange={`L${diff.line_number_b}`}
              code={diff.fragment_b_code}
              variant={CODE_BLOCK_VARIANTS.ADDED}
              showCopy={true}
              emptyPlaceholder="<empty>"
            />
          </div>
        ))}
      </div>
    </CollapsibleCard>
  );
};
