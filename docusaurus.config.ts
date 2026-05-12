import { themes as prismThemes } from 'prism-react-renderer';
import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';

const config: Config = {
  title: 'Kushaj',

  // Serve URL = ${url}/${baseUrl}
  // If hosting at "url", use baseUrl: '/'
  url: 'https://kushajveersingh.github.io',
  baseUrl: '/website',

  favicon: 'img/favicon.ico',
  trailingSlash: false, // For GitHub Pages
  i18n: {
    defaultLocale: 'en-US',
    locales: ['en-US'],
  },

  future: {
    v4: true,
  },

  onBrokenLinks: 'throw',
  onBrokenAnchors: 'throw',
  onDuplicateRoutes: 'throw',

  tagline: 'Kushajveer Singh Sooch personal website',

  // Used by "docusaurus deploy" command only
  organizationName: 'KushajveerSingh',
  projectName: 'website',

  themeConfig: {
    colorMode: {
      respectPrefersColorScheme: true,
    },
    docs: {
      sidebar: {
        hideable: true,
        autoCollapseCategories: true,
      },
    },
    navbar: {
      title: 'Kushaj',
      hideOnScroll: true,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: [],
      magicComments: [
        {
          className: 'theme-code-block-highlighted-line',
          line: 'highlight-next-line',
          block: { start: 'highlight-start', end: 'highlight-end' },
        },
        {
          className: 'theme-code-block-note-line',
          line: 'note-next-line',
          block: { start: 'note-start', end: 'note-end' },
        },
        {
          className: 'theme-code-block-tip-line',
          line: 'tip-next-line',
          block: { start: 'tip-start', end: 'tip-end' },
        },
        {
          className: 'theme-code-block-info-line',
          line: 'info-next-line',
          block: { start: 'info-start', end: 'info-end' },
        },
        {
          className: 'theme-code-block-warning-line',
          line: 'warning-next-line',
          block: { start: 'warning-start', end: 'warning-end' },
        },
        {
          className: 'theme-code-block-danger-line',
          line: 'danger-next-line',
          block: { start: 'danger-start', end: 'danger-end' },
        },
        {
          className: 'theme-code-block-added-line',
          line: 'added-next-line',
          block: { start: 'added-start', end: 'added-end' },
        },
        {
          className: 'theme-code-block-removed-line',
          line: 'removed-next-line',
          block: { start: 'removed-start', end: 'removed-end' },
        },
      ],
    },
    tableOfContents: {
      minHeadingLevel: 2,
      maxHeadingLevel: 4,
    },
  } satisfies Preset.ThemeConfig,

  plugins: [],
  themes: ['@docusaurus/theme-mermaid'],

  presets: [
    [
      'classic',
      {
        docs: {
          path: 'docs',
          editUrl: 'https://github.com/KushajveerSingh/website/tree/main/',
          routeBasePath: '/',
          tagsBasePath: 'tags',
          sidebarPath: './sidebars.ts',
          sidebarCollapsible: true,
          sidebarCollapsed: true,

          remarkPlugins: [remarkMath],
          rehypePlugins: [rehypeKatex],

          showLastUpdateTime: true,
          breadcrumbs: false,
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  markdown: {
    mermaid: true,
    hooks: {
      onBrokenMarkdownImages: 'throw',
      onBrokenMarkdownLinks: 'throw',
    },
  },

  headTags: [],
  scripts: [],
  stylesheets: [
    {
      href: 'https://cdn.jsdelivr.net/npm/katex@0.13.24/dist/katex.min.css',
      type: 'text/css',
      integrity:
        'sha384-odtC+0UGzzFL/6PNoE8rX/SPcQDXBJ+uRepguP4QkPCm2LBxH3FA3y+fKSiJ+AmM',
      crossorigin: 'anonymous',
    },
  ],
};

export default config;
