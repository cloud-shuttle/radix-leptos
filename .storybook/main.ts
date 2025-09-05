import type { StorybookConfig } from '@storybook/html-vite';

const config: StorybookConfig = {
  stories: ['../stories/**/*.stories.@(js|jsx|mjs|ts|tsx)'],
  addons: [
    '@storybook/addon-links',
    '@storybook/addon-essentials',
    '@storybook/addon-interactions',
    '@storybook/addon-a11y',
    '@storybook/addon-docs',
  ],
  framework: {
    name: '@storybook/html-vite',
    options: {},
  },
  docs: {
    autodocs: 'tag',
  },
  staticDirs: ['../examples', '../production-server'],
  viteFinal: async (config) => {
    // Add support for WASM files
    config.assetsInclude = ['**/*.wasm'];
    
    // Configure server for development
    if (config.server) {
      config.server.fs = {
        allow: ['..']
      };
    }
    
    return config;
  },
};

export default config;

