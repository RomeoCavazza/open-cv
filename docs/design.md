---
name: Institutional Minimalist
colors:
  surface: '#f9f9f9'
  surface-dim: '#dadada'
  surface-bright: '#f9f9f9'
  surface-container-lowest: '#ffffff'
  surface-container-low: '#f3f3f4'
  surface-container: '#eeeeee'
  surface-container-high: '#e8e8e8'
  surface-container-highest: '#e2e2e2'
  on-surface: '#1a1c1c'
  on-surface-variant: '#434656'
  inverse-surface: '#2f3131'
  inverse-on-surface: '#f0f1f1'
  outline: '#737688'
  outline-variant: '#c3c5d9'
  surface-tint: '#004ced'
  primary: '#0052ff'
  on-primary: '#ffffff'
  primary-container: '#0052ff'
  on-primary-container: '#dfe3ff'
  inverse-primary: '#b7c4ff'
  secondary: '#5e5e61'
  on-secondary: '#ffffff'
  secondary-container: '#e3e2e5'
  on-secondary-container: '#646467'
  tertiary: '#4c4e4f'
  on-tertiary: '#ffffff'
  tertiary-container: '#656666'
  on-tertiary-container: '#e4e4e4'
  error: '#ba1a1a'
  on-error: '#ffffff'
  error-container: '#ffdad6'
  on-error-container: '#93000a'
  primary-fixed: '#dde1ff'
  primary-fixed-dim: '#b7c4ff'
  on-primary-fixed: '#001452'
  on-primary-fixed-variant: '#0038b6'
  secondary-fixed: '#e3e2e5'
  secondary-fixed-dim: '#c7c6c9'
  on-secondary-fixed: '#1b1c1e'
  on-secondary-fixed-variant: '#464749'
  tertiary-fixed: '#e2e2e2'
  tertiary-fixed-dim: '#c6c6c7'
  on-tertiary-fixed: '#1a1c1c'
  on-tertiary-fixed-variant: '#454747'
  background: '#f9f9f9'
  on-background: '#1a1c1c'
  surface-variant: '#e2e2e2'
typography:
  display-lg:
    fontFamily: Inter
    fontSize: 64px
    fontWeight: '400'
    lineHeight: '1.1'
    letterSpacing: -0.02em
  display-md:
    fontFamily: Inter
    fontSize: 48px
    fontWeight: '400'
    lineHeight: '1.2'
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Inter
    fontSize: 32px
    fontWeight: '400'
    lineHeight: '1.3'
    letterSpacing: -0.02em
  headline-md:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '400'
    lineHeight: '1.4'
    letterSpacing: -0.02em
  body-lg:
    fontFamily: Inter
    fontSize: 18px
    fontWeight: '400'
    lineHeight: '1.6'
    letterSpacing: 0em
  body-md:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: '1.6'
    letterSpacing: 0em
  data-lg:
    fontFamily: JetBrains Mono
    fontSize: 16px
    fontWeight: '400'
    lineHeight: '1.5'
    letterSpacing: 0em
  data-sm:
    fontFamily: JetBrains Mono
    fontSize: 14px
    fontWeight: '400'
    lineHeight: '1.5'
    letterSpacing: 0em
  label-caps:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '600'
    lineHeight: '1.2'
    letterSpacing: 0.05em
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  base: 8px
  xs: 4px
  sm: 12px
  md: 24px
  lg: 48px
  xl: 80px
  section: 120px
  gutter: 24px
  margin: 48px
---

## Brand & Style

The design system is rooted in "Institutional Minimalism"—a style that balances high-utility financial tools with a sophisticated editorial aesthetic. It is designed to evoke a sense of calm authority, prioritizing clarity and ease of use to build long-term trust. 

The visual narrative is driven by an expansive use of whitespace, a rigorous adherence to a grid, and a focus on high-quality typography. By stripping away unnecessary ornamentation and relying on hairline borders and tonal shifts, the design system achieves a professional, "quiet luxury" feel appropriate for the global financial sector.

Note: this file is a visual direction reference. It is intentionally aspirational and may be stricter than the current implementation in `web/`.

## Colors

The color palette is intentionally restrained to maximize the impact of primary actions. 

- **Canvas**: The primary background is a pure white (#ffffff), providing a clean, high-contrast foundation.
- **Primary Action**: "Coinbase Blue" (#0052ff) is used for all primary buttons, progress indicators, and active states. 
- **Typography & Ink**: Headlines and heavy-weight text utilize Ink Black (#0a0b0d) to ensure maximum legibility and an authoritative presence.
- **Surface Elevation**: A soft gray (#f7f7f7) is used for structural bands, separating content sections without the need for heavy borders.

For the dark editorial hero sections, the palette inverts, using Ink Black as the canvas and White for primary typography, while retaining the "Coinbase Blue" for focal points.

## Typography

This design system utilizes a dual-font approach to distinguish between narrative content and technical data.

**Inter** is the workhorse font, used for both display and body text. To achieve an editorial look, all headlines and display styles are set at weight 400 with a tight -0.02em letter-spacing. This creates a refined, modern "Swiss" feel.

**JetBrains Mono** is used exclusively for tabular data, currency amounts, transaction hashes, and numerical labels. This ensures that character widths are consistent across changing values, which is critical for financial accuracy and a technical, "pro-tool" aesthetic.

## Layout & Spacing

The layout philosophy follows a **fixed grid** model to provide a stable, predictable reading experience. A 12-column grid is used for desktop views, with generous 48px margins to allow content to "breathe."

Vertical rhythm is established through "Elevation Bands"—full-width horizontal sections alternating between White and Soft Gray (#f7f7f7). This replaces the need for heavy dividers. Spacing between major sections should be expansive (120px) to maintain the editorial tone, while internal card padding remains a consistent 24px.

## Elevation & Depth

This design system avoids traditional drop shadows in favor of **Tonal Layers** and **Low-Contrast Outlines**.

- **Surface Tiers**: Depth is primarily indicated by placing white cards on top of the soft gray elevation bands. 
- **Hairline Borders**: Elements are defined by 1px hairline borders (using a subtle gray like #eceff1) rather than shadows. 
- **The "Floating" Exception**: In the dark editorial hero sections, product-UI mockup cards use a subtle, extra-diffused ambient shadow to create a sense of three-dimensional space, making them appear to float above the ink-black background.

## Shapes

The geometric language is a signature of this design system, using distinct radii to categorize elements.

- **Standard Elements**: UI elements like inputs and smaller containers use a standard 8px (0.5rem) radius.
- **Cards**: Large informational containers and product cards must use a **24px (1.5rem)** radius. This large radius creates a friendly, approachable contrast to the sharp typography.
- **Buttons & Pills**: All buttons and status badges are fully pill-shaped (100px radius), creating a distinct visual "click" target.
- **Icons**: Icons are always housed within a full circle container to maintain consistent visual weight.

## Components

### Buttons
Primary buttons are pill-shaped, using the Coinbase Blue background with white text. Secondary buttons should use a 1px hairline border in gray with Ink Black text. Hover states are subtle: a slight darkening of the blue or a faint gray background for secondary buttons.

### Cards
Cards are the primary container for all financial data. They feature a 24px corner radius, a 1px hairline border, and no shadow. Cards should be placed on a Soft Gray background to create a "lifted" appearance through color contrast alone.

### Input Fields
Inputs follow a minimal design: a 1px bottom border or a light hairline frame. They use Inter for labels and JetBrains Mono for the input text (especially for currency amounts).

### Lists & Tables
Financial tables use JetBrains Mono for all numeric values. Row heights are generous (56px minimum) to prevent visual clutter, with thin dividers separating entries.

### Dark Hero Sections
A specialized component for landing pages. The background is Ink Black (#0a0b0d). It contains large Display typography in White and features "Floating UI" cards—smaller card components with 24px radii that use subtle shadows to appear as if they are suspended in front of the background.
