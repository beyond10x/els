import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import {withProductSite} from '@beyond10x/docs-system/product-site';

const config: Config = {
  title: 'ELS',
  tagline:
    'Engineering protocols on Canon: which claims must hold, which evidence counts and for which revision, which actions need authority, and which outcomes a case can end in.',

  future: {
    v4: true,
  },

  url: 'https://beyond10x.github.io',
  baseUrl: '/els/',

  organizationName: 'beyond10x',
  projectName: 'els',
  trailingSlash: false,

  onBrokenLinks: 'throw',
  onBrokenAnchors: 'throw',

  markdown: {
    format: 'detect',
    hooks: {
      onBrokenMarkdownLinks: 'throw',
    },
  },

  plugins: [
    [
      '@docusaurus/plugin-client-redirects',
      {
        // Paths are relative to baseUrl. The documentation moved under /docs/ when the landing page
        // took /, so every earlier protocol and vocabulary URL keeps resolving.
        createRedirects(existingPath: string) {
          if (existingPath === '/docs/protocols' || existingPath.startsWith('/docs/protocols/')) {
            return [existingPath.replace(/^\/docs\//, '/')];
          }
          if (existingPath === '/docs/vocabulary') return ['/vocabulary'];
          return undefined;
        },
      },
    ],
  ],

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          routeBasePath: 'docs',
          editUrl: 'https://github.com/beyond10x/els/tree/main/website/',
        },
        blog: false,
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    navbar: {
      title: 'ELS',
      items: [
        {to: '/docs/', label: 'Documentation', position: 'left', activeBaseRegex: '^/els/docs/$'},
        {to: '/docs/category/concepts', label: 'Concepts', position: 'left'},
        {to: '/docs/protocols', label: 'Protocols', position: 'left'},
        {to: '/docs/vocabulary', label: 'Vocabulary', position: 'left'},
        {to: '/docs/showcase', label: 'Showcase', position: 'left'},
        {href: 'https://github.com/beyond10x/els', label: 'GitHub ↗', position: 'right'},
      ],
    },
    footer: {
      links: [
        {
          title: 'ELS',
          items: [
            {label: 'What ELS is', to: '/docs/'},
            {label: 'Concepts', to: '/docs/category/concepts'},
            {label: 'Protocols', to: '/docs/protocols'},
            {label: 'Vocabulary', to: '/docs/vocabulary'},
            {label: 'Showcase', to: '/docs/showcase'},
            {label: 'Source', href: 'https://github.com/beyond10x/els'},
          ],
        },
        {
          title: 'Family',
          items: [
            {label: 'Canon', href: 'https://github.com/beyond10x/canon'},
            {label: 'Commission', href: 'https://beyond10x.github.io/commission/'},
            {label: 'Loom', href: 'https://beyond10x.github.io/loom/'},
            {label: 'ESS', href: 'https://beyond10x.github.io/ess/'},
          ],
        },
      ],
      copyright: 'A beyond10x project. ELS · Apache-2.0.',
    },
  } satisfies Preset.ThemeConfig,
};

export default withProductSite(config, {landing: './product.json', mark: 'E'});
