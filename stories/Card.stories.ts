import type { Meta, StoryObj } from '@storybook/html';
import { html } from 'lit';

const meta: Meta = {
  title: 'Components/Card',
  component: 'div',
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: 'A flexible card component for displaying content with header, body, and footer sections.',
      },
    },
  },
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['default', 'outlined', 'elevated'],
      description: 'The visual style variant of the card',
    },
    padding: {
      control: { type: 'select' },
      options: ['none', 'sm', 'md', 'lg'],
      description: 'The padding size of the card',
    },
    showHeader: {
      control: { type: 'boolean' },
      description: 'Whether to show the card header',
    },
    showFooter: {
      control: { type: 'boolean' },
      description: 'Whether to show the card footer',
    },
  },
};

export default meta;
type Story = StoryObj;

export const Default: Story = {
  args: {
    variant: 'default',
    padding: 'md',
    showHeader: true,
    showFooter: true,
  },
  render: (args) => html`
    <div class="rounded-lg border bg-card text-card-foreground shadow-sm w-80">
      ${args.showHeader ? html`
        <div class="flex flex-col space-y-1.5 p-6">
          <h3 class="text-2xl font-semibold leading-none tracking-tight">Card Title</h3>
          <p class="text-sm text-muted-foreground">Card description</p>
        </div>
      ` : ''}
      <div class="p-6 pt-0">
        <p class="text-sm text-muted-foreground">
          This is the main content of the card. It can contain any type of content including text, images, forms, or other components.
        </p>
      </div>
      ${args.showFooter ? html`
        <div class="flex items-center p-6 pt-0">
          <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2">
            Action
          </button>
        </div>
      ` : ''}
    </div>
  `,
};

export const Outlined: Story = {
  args: {
    variant: 'outlined',
    padding: 'md',
    showHeader: true,
    showFooter: false,
  },
  render: (args) => html`
    <div class="rounded-lg border-2 border-border bg-card text-card-foreground w-80">
      ${args.showHeader ? html`
        <div class="flex flex-col space-y-1.5 p-6">
          <h3 class="text-2xl font-semibold leading-none tracking-tight">Outlined Card</h3>
          <p class="text-sm text-muted-foreground">With a prominent border</p>
        </div>
      ` : ''}
      <div class="p-6 pt-0">
        <p class="text-sm text-muted-foreground">
          This card has a more prominent border to draw attention to its content.
        </p>
      </div>
    </div>
  `,
};

export const Elevated: Story = {
  args: {
    variant: 'elevated',
    padding: 'lg',
    showHeader: true,
    showFooter: true,
  },
  render: (args) => html`
    <div class="rounded-lg border bg-card text-card-foreground shadow-lg w-80">
      ${args.showHeader ? html`
        <div class="flex flex-col space-y-1.5 p-8">
          <h3 class="text-2xl font-semibold leading-none tracking-tight">Elevated Card</h3>
          <p class="text-sm text-muted-foreground">With enhanced shadow</p>
        </div>
      ` : ''}
      <div class="p-8 pt-0">
        <p class="text-sm text-muted-foreground">
          This card has an enhanced shadow to create a floating effect above the page.
        </p>
      </div>
      ${args.showFooter ? html`
        <div class="flex items-center p-8 pt-0">
          <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2">
            Get Started
          </button>
        </div>
      ` : ''}
    </div>
  `,
};

export const NoHeader: Story = {
  args: {
    variant: 'default',
    padding: 'md',
    showHeader: false,
    showFooter: true,
  },
  render: (args) => html`
    <div class="rounded-lg border bg-card text-card-foreground shadow-sm w-80">
      <div class="p-6">
        <p class="text-sm text-muted-foreground">
          This card doesn't have a header, just content and a footer.
        </p>
      </div>
      ${args.showFooter ? html`
        <div class="flex items-center p-6 pt-0">
          <button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2">
            Learn More
          </button>
        </div>
      ` : ''}
    </div>
  `,
};

export const Compact: Story = {
  args: {
    variant: 'default',
    padding: 'sm',
    showHeader: true,
    showFooter: false,
  },
  render: (args) => html`
    <div class="rounded-lg border bg-card text-card-foreground shadow-sm w-80">
      ${args.showHeader ? html`
        <div class="flex flex-col space-y-1.5 p-4">
          <h3 class="text-lg font-semibold leading-none tracking-tight">Compact Card</h3>
          <p class="text-xs text-muted-foreground">Minimal padding</p>
        </div>
      ` : ''}
      <div class="p-4 pt-0">
        <p class="text-sm text-muted-foreground">
          This card uses minimal padding for a more compact appearance.
        </p>
      </div>
    </div>
  `,
};

