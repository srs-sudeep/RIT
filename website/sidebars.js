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
        'commands/write-tree',
        'commands/ls-tree',
        'commands/commit-tree',
        'commands/log',
        'commands/add',
        'commands/commit',
      ],
    },
  ],
};

module.exports = sidebars;
