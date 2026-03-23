# Weak Model Optimization Framework: "Rail-Guided Reasoner"

When using low-intelligence models (e.g., GPT-2/3 scale, Llama-3-8B-Lite), ADA switches to a **Micro-Atomic Execution Protocol**.

## 1. Action-Observation Loop (Strict Template)
Instead of asking for a "Plan," we force the model into a rigid `State -> Thought -> Action -> Observation` loop.
- **Thought**: Must be < 20 words.
- **Action**: Must be a single Tool Call from a pre-defined list.
- **Result**: The system provides the observation, then re-prompts.

## 2. Iterative Self-Correction (Reflection)
If a weak model produces an invalid JSON/YAML, a **Fixer-Prompt** is triggered:
> "Your previous output was invalid. Here is the exact error. Correct only the syntax."
This offloads logic from the reasoning phase to a pure syntax-fixing phase.

## 3. Knowledge Injection (Few-Shot RAG)
Weak models suffer from "forgetting" instructions. ADA solves this by:
- **Per-Step Instruction**: Re-sending the system prompt *every single round*.
- **Plan Anchoring**: Always including the "Original Goal" and "Last 2 Steps" as the only history (Context Pruning).

## 4. Master-Validator Architecture
A slightly better model (even a local SLM like Phi-3) acts as the **Director**, while the weak models act as **Action Workers**. 
- The Director writes the plan.
- The Weak Model executes ONE line of the plan.

## Conclusion
Even with "GPT-2 level" models, ADA can succeed by **Maximum Instruction Density** and **Minimum Action Granularity**.
