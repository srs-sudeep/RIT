#!/bin/bash
# Setup Docusaurus documentation site for Rit

set -e

echo "🦖 Setting up Docusaurus documentation..."

# Navigate to project root
cd "$(dirname "$0")/.."

# Create Docusaurus site (if it doesn't exist)
if [ ! -d "website" ]; then
    npx create-docusaurus@latest website classic --typescript
fi

# Documentation files should be in website/docs/
# Edit files directly in website/docs/ folder

# Create sidebars config
cat > website/sidebars.js << 'EOF'
/**
 * @type {import('@docusaurus/plugin-content-docs').SidebarsConfig}
 */
const sidebars = {
  docs: [
    'intro',
    'architecture',
    {
      type: 'category',
      label: 'Commands',
      items: [
        'commands/init',
        'commands/hash-object',
        'commands/cat-file',
      ],
    },
  ],
};

module.exports = sidebars;
EOF

# Update docusaurus config
cat > website/docusaurus.config.ts << 'EOF'
import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Rit',
  tagline: 'A Git Implementation in Rust',
  favicon: 'img/favicon.ico',
  url: 'https://your-username.github.io',
  baseUrl: '/rit/',
  organizationName: 'your-username',
  projectName: 'rit',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },
  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.js',
          editUrl: 'https://github.com/your-username/rit/tree/main/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],
  themeConfig: {
    navbar: {
      title: 'Rit',
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docs',
          position: 'left',
          label: 'Documentation',
        },
        {
          href: 'https://github.com/your-username/rit',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      copyright: `Built with Rust and ❤️`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['rust', 'bash'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
EOF

echo "✅ Docusaurus setup complete!"
echo ""
echo "To start the documentation server:"
echo "  cd website && npm start"
echo ""
echo "To build for production:"
echo "  cd website && npm run build"

