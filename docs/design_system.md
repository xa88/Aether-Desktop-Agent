# ADA Design System (UI/UX Pro Max)

This document outlines the official design system and token structure for the Aether Desktop Agent.

## Color Palette

### Neutrals
- `neutral-50` to `neutral-900`: Used for backgrounds, surface borders, and typography.
  
### Primary (Brand)
- `blue-500` (#3B82F6): Primary brand color for buttons, links, and active states.

### State Colors
- **Success** (#10B981): Affirmative actions, completion, cache hits.
- **Warning** (#F59E0B): Non-blocking alerts, retries.
- **Error** (#EF4444): Blocking errors, exceptions, crashes.
- **Info** (#6366F1): Contextual information.
- **Risk** (#8B5CF6): High-impact agent actions requiring user consent.

## Typography
- **Primary Font**: `Inter`, `-apple-system`, `BlinkMacSystemFont`, `Segoe UI`, `Roboto`, `sans-serif`
- **Monospace**: `JetBrains Mono`, `Fira Code`, `Consolas`, `monospace`

## Spacing & Radii
- **Scale**: Multiples of 4px (4px, 8px, 12px, 16px, 24px, 32px)
- **Border Radius**: 
  - `sm` (4px): Small badges, inputs.
  - `md` (8px): Standard buttons, cards.
  - `lg` (12px): Modals, dialogs.

## CSS Tokens Usage
Standardized tokens are available via `--ada-*` CSS variables in `tokens.css`. Always use these variables for colors, spacing, and typography to ensure consistency and theme support.
