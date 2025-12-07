import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Rit',
  tagline: 'A Git Implementation in Rust',
  favicon: 'img/rit-logo-small.svg',
  url: 'https://srs-sudeep.github.io',
  baseUrl: '/rit/',
  organizationName: 'srs-sudeep',
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
          path: './docs', // Use website/docs folder
          sidebarPath: './sidebars.js',
          editUrl: 'https://github.com/srs-sudeep/rit/tree/main/website/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],
  themeConfig: {
    image: 'img/rit-logo.svg',
    navbar: {
      title: 'Rit',
      logo: {
        alt: 'Rit Logo',
        src: 'img/rit-logo-small.svg',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docs',
          position: 'left',
          label: 'Documentation',
        },
        {
          href: 'https://github.com/srs-sudeep/rit',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      copyright: `Copyright © ${new Date().getFullYear()} Sudeep Ranjan Sahoo. Built with Rust and ❤️`,
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Introduction',
              to: '/docs/intro',
            },
            {
              label: 'Architecture',
              to: '/docs/architecture',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/srs-sudeep/rit',
            },
            {
              label: 'Issues',
              href: 'https://github.com/srs-sudeep/rit/issues',
            },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'LinkedIn',
              href: 'https://www.linkedin.com/in/sudeep-ranjan-sahoo-b82355232/',
            },
            {
              label: 'Twitter/X',
              href: 'https://x.com/SUDEEPRANJANSA1',
            },
          ],
        },
      ],
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['rust', 'bash'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
