# OGP Image Generator

Generates OGP (Open Graph Protocol) images for gitlogue using [satori](https://github.com/vercel/satori).

## Prerequisites

- Node.js 18+
- System fonts (DejaVu Sans)

## Usage

```bash
npm install
npm run generate
```

This will generate `docs/assets/ogp.png` (1200x630px) suitable for GitHub's social preview.

## How it works

1. Loads `screenshot-editor.png` as the background
2. Applies a dark overlay for text legibility
3. Renders text using satori (HTML/CSS to SVG)
4. Converts SVG to PNG using @resvg/resvg-js

## Customization

Edit `generate.js` to customize:
- Title and description text
- Colors and styling
- Font sizes
- Background image
