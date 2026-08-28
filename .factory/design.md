# Visual thesis — blueprint drafting sheet

iOS Review Gate looks like a release engineer's marked-up drafting sheet. The page is warm stock with a precise navy grid. Vermilion stamps mark decisions. This makes the product feel inspectable and procedural, not like another build dashboard.

## Tokens

- Paper `#F3EEDC`; raised paper `#FFFDF5`; blueprint navy `#102D4F`; ink `#142338`; muted ink `#536171`.
- Action blue `#0B57A3` with white; pass green `#17633B`; warning ochre `#7B4E00`; reject vermilion `#A72D25`.
- Dark treatment uses navy `#0C1B2A`, drafting surface `#132A3F`, chalk `#F5F0DF`, and signal cyan `#7EC8E3`.
- All body and control combinations target WCAG AA contrast. State also uses words and shapes.

## Type and spacing

- Display and field labels use the self-hosted B612 Mono family. The font is licensed under the SIL Open Font License. Body text uses the native UI sans stack. The pairing resembles technical annotations without hurting reading speed.
- Spacing follows an 8 px base: 4, 8, 16, 24, 32, 48, 72, 96. Text measure stops at 68 characters.
- Hairlines, corner ticks, punched circles, and rectangular stamps form the shape language. Corners stay at 2–6 px.

## Interaction and motion

- Controls move down by 2 px when pressed, like a physical stamp.
- The hero's inspection line makes one 700 ms pass when the page loads. Nothing loops. Route changes fade for 180 ms.
- With reduced motion, all movement is removed and state changes are immediate.
- Focus uses a 3 px vermilion ring with a paper-colored offset.

## Responsive intent

Desktop shows the packet preview beside its annotations. At 390 px, the sheet becomes a single column; small drafting notes disappear, but the result, command, and primary action remain. Targets remain at least 44 px.

## Asset plan and provenance

- `site/public/assets/release-blueprint.webp`: original raster illustration generated for this product with `/opt/fleet/lib/gen-image.sh` using the factory image deployment on 2026-08-28. Prompt: “Editorial technical blueprint illustration on warm ivory drafting paper, exploded axonometric view of an iPhone app release packet: archive box, metadata sheets, screenshot contact sheet, privacy manifest checklist, and a queue timeline aligned by precise navy construction lines; one small vermilion approval stamp; sparse, refined, tactile paper grain, screen-print ink, no people, no logos, no readable text, no gradient, wide landscape composition.” Optimized locally to WebP. The generated PNG source is kept at `.factory/assets/release-blueprint-source.png`.
- `site/public/assets/b612-mono.woff2`: self-hosted subset of B612 Mono Regular. The original TTF is kept at `.factory/assets/b612-mono-source.ttf`; B612 is Copyright 2012 Airbus and licensed under the SIL Open Font License 1.1.
- `og-card.svg`, favicon, and UI symbols are hand-made SVG/code assets. They use only geometric drafting marks and product text.

No stock assets, external fonts, or CDN resources are used.
