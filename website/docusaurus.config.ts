import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import docsSystemPlugin, {ecosystemFooterGroup, ecosystemNavbarItems} from '@beyond10x/docs-system/docusaurus';

const MONO_STACK =
  "ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, 'DejaVu Sans Mono', 'Liberation Mono', monospace";

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

  markdown: {
    format: 'detect',
    hooks: {
      onBrokenMarkdownLinks: 'throw',
    },
    mermaid: true,
  },

  themes: ['@docusaurus/theme-mermaid'],
  plugins: [docsSystemPlugin],

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
          routeBasePath: '/',
          editUrl: 'https://github.com/beyond10x/els/tree/main/website/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    colorMode: {
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'ELS',
      items: [
        ...ecosystemNavbarItems(),
        {type: 'docSidebar', sidebarId: 'docsSidebar', position: 'left', label: 'Documentation'},
        {to: '/protocols', label: 'Protocols', position: 'left'},
        {to: '/vocabulary', label: 'Vocabulary', position: 'left'},
        {href: 'https://github.com/beyond10x/els', label: 'GitHub', position: 'right'},
      ],
    },
    footer: {
      style: 'dark',
      links: [
        ecosystemFooterGroup(),
        {
          title: 'Documentation',
          items: [
            {label: 'What ELS is', to: '/'},
            {label: 'Protocols', to: '/protocols'},
            {label: 'Vocabulary', to: '/vocabulary'},
          ],
        },
        {
          title: 'Project',
          items: [
            {label: 'Source', href: 'https://github.com/beyond10x/els'},
            {label: 'Canon', href: 'https://github.com/beyond10x/canon'},
          ],
        },
      ],
      copyright: 'ELS · Apache-2.0 · built with Docusaurus.',
    },
    prism: {
      additionalLanguages: ['yaml', 'bash'],
    },
    mermaid: {
      theme: {light: 'neutral', dark: 'dark'},
      options: {
        fontFamily: MONO_STACK,
      },
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
