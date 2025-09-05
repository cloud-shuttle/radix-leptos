import type { Meta, StoryObj } from '@storybook/html';
import { html } from 'lit';

const meta: Meta = {
  title: 'Components/Button',
  component: 'button',
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: 'A versatile button component with multiple variants and sizes.',
      },
    },
  },
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['default', 'destructive', 'outline', 'secondary', 'ghost', 'link'],
      description: 'The visual style variant of the button',
    },
    size: {
      control: { type: 'select' },
      options: ['default', 'sm', 'lg', 'icon'],
      description: 'The size of the button',
    },
    disabled: {
      control: { type: 'boolean' },
      description: 'Whether the button is disabled',
    },
    children: {
      control: { type: 'text' },
      description: 'The button text content',
    },
  },
};

export default meta;
type Story = StoryObj;

export const Default: Story = {
  args: {
    children: 'Button',
    variant: 'default',
    size: 'default',
    disabled: false,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

export const Destructive: Story = {
  args: {
    children: 'Delete',
    variant: 'destructive',
    size: 'default',
    disabled: false,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-destructive text-destructive-foreground hover:bg-destructive/90 h-10 px-4 py-2"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

export const Outline: Story = {
  args: {
    children: 'Outline',
    variant: 'outline',
    size: 'default',
    disabled: false,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

export const Secondary: Story = {
  args: {
    children: 'Secondary',
    variant: 'secondary',
    size: 'default',
    disabled: false,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-secondary text-secondary-foreground hover:bg-secondary/80 h-10 px-4 py-2"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

export const Ghost: Story = {
  args: {
    children: 'Ghost',
    variant: 'ghost',
    size: 'default',
    disabled: false,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 px-4 py-2"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

export const Link: Story = {
  args: {
    children: 'Link',
    variant: 'link',
    size: 'default',
    disabled: false,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 text-primary underline-offset-4 hover:underline h-10 px-4 py-2"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

export const Small: Story = {
  args: {
    children: 'Small',
    variant: 'default',
    size: 'sm',
    disabled: false,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 rounded-md px-3"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

export const Large: Story = {
  args: {
    children: 'Large',
    variant: 'default',
    size: 'lg',
    disabled: false,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-11 rounded-md px-8"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

export const Icon: Story = {
  args: {
    children: '⚙️',
    variant: 'default',
    size: 'icon',
    disabled: false,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 w-10"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

export const Disabled: Story = {
  args: {
    children: 'Disabled',
    variant: 'default',
    size: 'default',
    disabled: true,
  },
  render: (args) => html`
    <button 
      class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
      ?disabled=${args.disabled}
    >
      ${args.children}
    </button>
  `,
};

