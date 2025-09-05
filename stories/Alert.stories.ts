import type { Meta, StoryObj } from '@storybook/html';
import { html } from 'lit';

const meta: Meta = {
  title: 'Components/Alert',
  component: 'div',
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: 'An alert component for displaying important messages to users with different variants.',
      },
    },
  },
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['default', 'destructive', 'warning', 'success'],
      description: 'The visual style variant of the alert',
    },
    size: {
      control: { type: 'select' },
      options: ['sm', 'md', 'lg'],
      description: 'The size of the alert',
    },
    showIcon: {
      control: { type: 'boolean' },
      description: 'Whether to show an icon in the alert',
    },
    dismissible: {
      control: { type: 'boolean' },
      description: 'Whether the alert can be dismissed',
    },
  },
};

export default meta;
type Story = StoryObj;

export const Default: Story = {
  args: {
    variant: 'default',
    size: 'md',
    showIcon: true,
    dismissible: false,
  },
  render: (args) => html`
    <div class="relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground">
      ${args.showIcon ? html`<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>` : ''}
      <div class="text-sm font-medium">Heads up!</div>
      <div class="text-sm [&_p]:leading-relaxed">
        You can add components to your app using the cli.
      </div>
    </div>
  `,
};

export const Destructive: Story = {
  args: {
    variant: 'destructive',
    size: 'md',
    showIcon: true,
    dismissible: false,
  },
  render: (args) => html`
    <div class="relative w-full rounded-lg border border-destructive/50 text-destructive dark:border-destructive [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-destructive">
      ${args.showIcon ? html`<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
      </svg>` : ''}
      <div class="text-sm font-medium">Error</div>
      <div class="text-sm [&_p]:leading-relaxed">
        Your session has expired. Please log in again.
      </div>
    </div>
  `,
};

export const Warning: Story = {
  args: {
    variant: 'warning',
    size: 'md',
    showIcon: true,
    dismissible: false,
  },
  render: (args) => html`
    <div class="relative w-full rounded-lg border border-yellow-500/50 text-yellow-800 dark:text-yellow-200 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-yellow-600 dark:[&>svg]:text-yellow-400">
      ${args.showIcon ? html`<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
      </svg>` : ''}
      <div class="text-sm font-medium">Warning</div>
      <div class="text-sm [&_p]:leading-relaxed">
        This action cannot be undone. Please proceed with caution.
      </div>
    </div>
  `,
};

export const Success: Story = {
  args: {
    variant: 'success',
    size: 'md',
    showIcon: true,
    dismissible: false,
  },
  render: (args) => html`
    <div class="relative w-full rounded-lg border border-green-500/50 text-green-800 dark:text-green-200 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-green-600 dark:[&>svg]:text-green-400">
      ${args.showIcon ? html`<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>` : ''}
      <div class="text-sm font-medium">Success</div>
      <div class="text-sm [&_p]:leading-relaxed">
        Your changes have been saved successfully.
      </div>
    </div>
  `,
};

export const Small: Story = {
  args: {
    variant: 'default',
    size: 'sm',
    showIcon: true,
    dismissible: false,
  },
  render: (args) => html`
    <div class="relative w-full rounded-lg border p-3 [&>svg~*]:pl-6 [&>svg+div]:translate-y-[-2px] [&>svg]:absolute [&>svg]:left-3 [&>svg]:top-3 [&>svg]:text-foreground">
      ${args.showIcon ? html`<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>` : ''}
      <div class="text-xs font-medium">Small Alert</div>
      <div class="text-xs [&_p]:leading-relaxed">
        This is a compact alert message.
      </div>
    </div>
  `,
};

export const Large: Story = {
  args: {
    variant: 'default',
    size: 'lg',
    showIcon: true,
    dismissible: false,
  },
  render: (args) => html`
    <div class="relative w-full rounded-lg border p-6 [&>svg~*]:pl-8 [&>svg+div]:translate-y-[-4px] [&>svg]:absolute [&>svg]:left-6 [&>svg]:top-6 [&>svg]:text-foreground">
      ${args.showIcon ? html`<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
      </svg>` : ''}
      <div class="text-base font-medium">Large Alert</div>
      <div class="text-base [&_p]:leading-relaxed">
        This is a larger alert with more prominent text and spacing.
      </div>
    </div>
  `,
};

export const NoIcon: Story = {
  args: {
    variant: 'default',
    size: 'md',
    showIcon: false,
    dismissible: false,
  },
  render: (args) => html`
    <div class="relative w-full rounded-lg border p-4">
      <div class="text-sm font-medium">Alert without Icon</div>
      <div class="text-sm [&_p]:leading-relaxed">
        This alert doesn't have an icon, just text content.
      </div>
    </div>
  `,
};

