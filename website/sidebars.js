/**
 * @type {import('@docusaurus/plugin-content-docs').SidebarsConfig}
 */
const sidebars = {
  docs: [
    'intro',
    'architecture',
    'ritignore',
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
        'commands/branch',
        'commands/checkout',
        'commands/tag',
        'commands/status',
        'commands/diff',
      ],
    },
  ],
};

module.exports = sidebars;
