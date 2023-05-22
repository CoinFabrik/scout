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
  url: 'https://your-docusaurus-test-site.com',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'CoinFabrik', // Usually your GitHub org/user name.
  projectName: 'Scout', // Usually your repo name.

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
            'https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/',
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
      image: 'img/docusaurus-social-card.jpg',
      navbar: {
        title: 'Scout',
        logo: {
          alt: 'Scout Logo',
          src: 'img/logo.svg',
        },
        items: [
          {type: 'docSidebar', sidebarId: 'docsSidebar', label: 'Docs', position: 'left'},
          {to: '/blog', label: 'Blog', position: 'left'},
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
                label: 'Learn',
                to: '/docs/Learn',
              },
              {
                label: 'Tutorials',
                to: '/docs/tutorials',
              },
              {
                label: 'Contribute',
                to: '/docs/contribute',
              },
              {
                label: 'FAQs',
                to: '/docs/faqs',
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
      prism: {
        theme: darkCodeTheme,
        darkTheme: darkCodeTheme,
      },
    }),
};

module.exports = config;
