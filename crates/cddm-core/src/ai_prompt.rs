use crate::types::{CloneType, LineSpan};
use serde::{Deserialize, Serialize};

/// Occurrence context for AI refactoring prompt generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiOccurrenceContext {
    /// File path
    pub path: String,
    /// Line range
    pub span: LineSpan,
    /// Code fragment content
    pub snippet: String,
}

/// Request parameters for synthesizing an AI deduplication prompt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiRefactorPromptRequest {
    /// Classification of the clone pair or cluster
    pub clone_type: CloneType,
    /// Similarity score (0.0 to 1.0)
    pub similarity: f64,
    /// Token count of the duplicated sequence
    pub token_count: usize,
    /// Estimated lines of code saved by deduplication
    pub lines_saved_est: usize,
    /// Proposed extracted function name
    pub function_name: String,
    /// Proposed target module path
    pub target_module: String,
    /// List of duplicated occurrence sites
    pub occurrences: Vec<AiOccurrenceContext>,
    /// Invariant consensus body extracted by LCS
    pub invariant_body: String,
    /// Identified parameter names / variable identifiers
    pub parameters: Vec<String>,
    /// Program Dependence Graph context slices capturing data dependencies
    #[serde(default)]
    pub context_slices: Option<Vec<crate::semantic_graph::ContextSlice>>,
    /// Optional custom user guidelines or architectural constraints
    pub custom_instructions: Option<String>,
}

/// Synthesizes an AI prompt for an AI assistant to refactor duplicated code.
pub fn generate_ai_refactor_prompt(req: &AiRefactorPromptRequest) -> String {
    let mut prompt = String::new();

    prompt.push_str("# Code De-Duplication & Refactoring Specification\n\n");
    prompt.push_str(
        "You are an expert software architect refactoring duplicated code to improve \
         maintainability and DRY compliance.\n\n",
    );

    prompt.push_str("## 1. Duplication Diagnostics\n\n");
    prompt.push_str(&format!(
        "- **Clone Classification**: {:?}\n",
        req.clone_type
    ));
    prompt.push_str(&format!(
        "- **Structural Similarity**: {:.1}%\n",
        req.similarity * 100.0
    ));
    prompt.push_str(&format!("- **Duplicate Tokens**: {}\n", req.token_count));
    prompt.push_str(&format!(
        "- **Estimated Lines Saved**: {} LOC\n",
        req.lines_saved_est
    ));
    prompt.push_str(&format!(
        "- **Target Function Name**: `{}`\n",
        req.function_name
    ));
    prompt.push_str(&format!(
        "- **Destination Module**: `{}`\n\n",
        req.target_module
    ));

    if !req.parameters.is_empty() {
        prompt.push_str("## 2. Identified Parameter Variations\n\n");
        prompt.push_str(
            "The following variable identifiers vary across clone occurrences and should be \
             parameterized:\n",
        );
        for param in &req.parameters {
            prompt.push_str(&format!("- `{}`\n", param));
        }
        prompt.push('\n');
    }

    if !req.invariant_body.trim().is_empty() {
        prompt.push_str("## 3. Extracted Consensus Invariant Logic\n\n");
        prompt.push_str("```\n");
        prompt.push_str(req.invariant_body.trim());
        prompt.push_str("\n```\n\n");
    }

    prompt.push_str("## 4. Duplicate Occurrence Sites\n\n");
    for (i, occ) in req.occurrences.iter().enumerate() {
        prompt.push_str(&format!(
            "### Occurrence {} - `{}:{}-{}`\n\n",
            i + 1,
            occ.path,
            occ.span.line_start,
            occ.span.line_end
        ));
        prompt.push_str("```\n");
        prompt.push_str(occ.snippet.trim());
        prompt.push_str("\n```\n\n");
    }

    if let Some(slices) = &req.context_slices
        && !slices.is_empty()
    {
        prompt.push_str("## 5. Sliced Context & Data Dependencies (PDG Static Program Slices)\n\n");
        for (i, slice) in slices.iter().enumerate() {
            prompt.push_str(&format!(
                "### Context Slice {} - Enclosing Function `{}` (Lines {}-{})\n\n",
                i + 1,
                slice.enclosing_function,
                slice.line_span.0,
                slice.line_span.1
            ));
            if !slice.required_variables.is_empty() {
                prompt.push_str(&format!(
                    "- **Required Upstream Variables**: `{}`\n",
                    slice.required_variables.join("`, `")
                ));
            }
            if !slice.defined_variables.is_empty() {
                prompt.push_str(&format!(
                    "- **Exported Downstream Variables**: `{}`\n",
                    slice.defined_variables.join("`, `")
                ));
            }
            if !slice.upstream_statements.is_empty() {
                prompt.push_str("- **Preceding Definitions**:\n");
                for stmt in &slice.upstream_statements {
                    prompt.push_str(&format!("  - `{}`\n", stmt));
                }
            }
            prompt.push('\n');
        }
    }

    if let Some(custom) = &req.custom_instructions
        && !custom.trim().is_empty()
    {
        prompt.push_str("## 6. Architectural Constraints\n\n");
        prompt.push_str(custom.trim());
        prompt.push_str("\n\n");
    }

    prompt.push_str("## Instructions for AI Assistant\n\n");
    prompt.push_str(
        "1. **Synthesize Shared Abstraction**: Define a clean, idiomatic shared function named `",
    );
    prompt.push_str(&req.function_name);
    prompt.push_str("` in `");
    prompt.push_str(&req.target_module);
    prompt.push_str(
        "` that encapsulates the common invariant logic with appropriate generic type signatures, \
         error handling, and docstrings.\n",
    );
    prompt.push_str(
        "2. **Refactor Call Sites**: Update each occurrence site listed above to import and \
         invoke the new shared function, passing site-specific arguments.\n",
    );
    prompt.push_str(
        "3. **Preserve Behavior**: Ensure zero semantic regressions, zero compiler/linter \
         warnings, and preserve existing unit test coverage.\n",
    );
    prompt.push_str(
        "4. **Provide Diff**: Output the unified Git `.patch` or complete modified files.\n",
    );

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ai_refactor_prompt() {
        let req = AiRefactorPromptRequest {
            clone_type: CloneType::Renamed,
            similarity: 0.95,
            token_count: 120,
            lines_saved_est: 24,
            function_name: "normalize_user_input".to_string(),
            target_module: "src/utils/normalization.rs".to_string(),
            occurrences: vec![
                AiOccurrenceContext {
                    path: "src/auth/login.rs".to_string(),
                    span: LineSpan {
                        line_start: 10,
                        line_end: 25,
                        byte_offset: 200,
                    },
                    snippet: "let user = input.trim().to_lowercase();".to_string(),
                },
                AiOccurrenceContext {
                    path: "src/auth/register.rs".to_string(),
                    span: LineSpan {
                        line_start: 30,
                        line_end: 45,
                        byte_offset: 600,
                    },
                    snippet: "let name = raw_name.trim().to_lowercase();".to_string(),
                },
            ],
            invariant_body: "let result = target.trim().to_lowercase();".to_string(),
            parameters: vec!["target".to_string()],
            context_slices: None,
            custom_instructions: Some("Do not introduce external dependencies.".to_string()),
        };

        let prompt = generate_ai_refactor_prompt(&req);
        assert!(prompt.contains("normalize_user_input"));
        assert!(prompt.contains("src/utils/normalization.rs"));
        assert!(prompt.contains("src/auth/login.rs:10-25"));
        assert!(prompt.contains("src/auth/register.rs:30-45"));
        assert!(prompt.contains("Do not introduce external dependencies."));
        assert!(prompt.contains("95.0%"));
    }

    #[test]
    fn test_generate_ai_refactor_prompt_with_context_slices() {
        let req = AiRefactorPromptRequest {
            clone_type: CloneType::NearMiss,
            similarity: 0.88,
            token_count: 90,
            lines_saved_est: 18,
            function_name: "calc_metrics".to_string(),
            target_module: "src/stats.rs".to_string(),
            occurrences: vec![],
            invariant_body: "let res = a * b;".to_string(),
            parameters: vec!["a".to_string(), "b".to_string()],
            context_slices: Some(vec![crate::semantic_graph::ContextSlice {
                enclosing_function: "process_handler".to_string(),
                line_span: (15, 20),
                defined_variables: vec!["res".to_string()],
                required_variables: vec!["a".to_string(), "b".to_string()],
                upstream_statements: vec!["let a = 10;".to_string()],
                downstream_statements: vec!["println!(\"{}\", res);".to_string()],
            }]),
            custom_instructions: None,
        };

        let prompt = generate_ai_refactor_prompt(&req);
        assert!(prompt.contains("PDG Static Program Slices"));
        assert!(prompt.contains("process_handler"));
        assert!(prompt.contains("Required Upstream Variables"));
    }
}
