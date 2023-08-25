// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Scout',
  tagline: 'Security Analysis Tool',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://coinfabrik.github.io',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/scout/',
  trailingSlash: false,

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'CoinFabrik', // Usually your GitHub org/user name.
  projectName: 'scout', // Usually your repo name.
  deploymentBranch: 'gh-pages', // The branch of your docs repo that you are going to deploy to GitHub pages.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/CoinFabrik/scout',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://blog.coinfabrik.com/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/scout-social-card.jpg',
      navbar: {
        title: 'Scout',
        logo: {
          alt: 'Scout Logo',
          src: 'img/scout.svg',
        },
        items: [
          {type: 'docSidebar', sidebarId: 'docsSidebar', label: 'Docs', position: 'left'},
          {to: 'https://blog.coinfabrik.com/', label: 'Blog', position: 'left'},
          {to: '/acknowledgements', label: 'Acknowledgements', position: 'left'},
          {to: '/about', label: 'About', position: 'left'},
          {
            href: 'https://github.com/CoinFabrik/scout',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Getting Started',
                to: '/docs/intro',
              },
              {
                label: 'Vulnerabilities',
                to: '/docs/vulnerabilities',
              },
              {
                label: 'Detectors',
                to: '/docs/detectors',
              },
              {
                label: 'Contribute',
                to: '/docs/contribute',
              },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'Twitter',
                href: 'https://twitter.com/coinfabrik?lang=en',
              },
              {
                label: 'Instagram',
                href: 'https://www.instagram.com/coinfabrik/',
              },
              {
                label: 'Reddit',
                href: 'https://www.reddit.com/r/CoinFabrik/',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'Web Site',
                to: 'https://www.coinfabrik.com/',
              },
              {
                label: 'Blog',
                to: 'https://blog.coinfabrik.com/',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/CoinFabrik',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Scout, CoinFabrik.`,
      },
      colorMode: {
        defaultMode: 'light',
        disableSwitch: true,
        respectPrefersColorScheme: false,
      },

      prism: {
        theme: darkCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['rust', 'toml'],
      },

    }),
};

module.exports = config;
