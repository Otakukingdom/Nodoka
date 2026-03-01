/**
 * Unit tests for Component base class
 *
 * Tests component lifecycle: mount, unmount, update, and lifecycle hooks
 */

import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { Component } from './Component';

// Create a concrete test component
class TestComponent extends Component {
  public renderCount = 0;
  public afterMountCount = 0;
  public beforeUnmountCount = 0;

  render(): HTMLElement {
    this.renderCount++;
    const div = document.createElement('div');
    div.className = 'test-component';
    div.textContent = `Render ${this.renderCount}`;
    return div;
  }

  protected override afterMount(): void {
    this.afterMountCount++;
  }

  protected override beforeUnmount(): void {
    this.beforeUnmountCount++;
  }

  // Expose protected method for testing
  public triggerUpdate(): void {
    this.update();
  }
}

describe('Component', () => {
  let component: TestComponent;
  let container: HTMLElement;

  beforeEach(() => {
    component = new TestComponent();
    container = document.createElement('div');
    document.body.appendChild(container);
  });

  afterEach(() => {
    container.remove();
  });

  describe('render', () => {
    it('should create an element when rendered', () => {
      const element = component.render();
      expect(element).toBeInstanceOf(HTMLElement);
      expect(element.className).toBe('test-component');
    });

    it('should increment render count on each render', () => {
      component.render();
      expect(component.renderCount).toBe(1);

      component.render();
      expect(component.renderCount).toBe(2);
    });
  });

  describe('mount', () => {
    it('should append element to parent', () => {
      component.mount(container);
      expect(container.children).toHaveLength(1);
      expect(container.firstElementChild?.className).toBe('test-component');
    });

    it('should call afterMount hook', () => {
      expect(component.afterMountCount).toBe(0);
      component.mount(container);
      expect(component.afterMountCount).toBe(1);
    });

    it('should return this for chaining', () => {
      const result = component.mount(container);
      expect(result).toBe(component);
    });
  });

  describe('unmount', () => {
    it('should remove element from DOM', () => {
      component.mount(container);
      expect(container.children).toHaveLength(1);

      component.unmount();
      expect(container.children).toHaveLength(0);
    });

    it('should call beforeUnmount hook', () => {
      component.mount(container);
      expect(component.beforeUnmountCount).toBe(0);

      component.unmount();
      expect(component.beforeUnmountCount).toBe(1);
    });

    it('should be safe to call when not mounted', () => {
      expect(() => component.unmount()).not.toThrow();
    });

    it('should be safe to call multiple times', () => {
      component.mount(container);
      component.unmount();
      component.unmount();
      component.unmount();
      expect(container.children).toHaveLength(0);
    });
  });

  describe('update', () => {
    it('should re-render the element', () => {
      component.mount(container);
      expect(component.renderCount).toBe(1);

      component.triggerUpdate();
      expect(component.renderCount).toBe(2);
    });

    it('should call lifecycle hooks during update', () => {
      component.mount(container);
      expect(component.afterMountCount).toBe(1);
      expect(component.beforeUnmountCount).toBe(0);

      component.triggerUpdate();
      expect(component.afterMountCount).toBe(2);
      expect(component.beforeUnmountCount).toBe(1);
    });

    it('should preserve element in DOM after update', () => {
      component.mount(container);
      const originalElement = container.firstElementChild;

      component.triggerUpdate();

      expect(container.children).toHaveLength(1);
      // Element should be different (re-rendered)
      expect(container.firstElementChild).not.toBe(originalElement);
    });

    it('should not update if not mounted', () => {
      component.triggerUpdate();
      expect(component.renderCount).toBe(0);
    });
  });

  describe('element reference', () => {
    it('should have null element before mount', () => {
      expect(component['element']).toBeNull();
    });

    it('should have element reference after mount', () => {
      component.mount(container);
      expect(component['element']).toBeInstanceOf(HTMLElement);
    });

    it('should have null element after unmount', () => {
      component.mount(container);
      component.unmount();
      expect(component['element']).toBeNull();
    });
  });
});

// Test a component with event handlers
class InteractiveComponent extends Component {
  public clickCount = 0;

  render(): HTMLElement {
    const button = document.createElement('button');
    button.textContent = 'Click me';
    button.addEventListener('click', () => this.handleClick());
    return button;
  }

  private handleClick(): void {
    this.clickCount++;
  }
}

describe('InteractiveComponent', () => {
  it('should handle click events', () => {
    const component = new InteractiveComponent();
    const container = document.createElement('div');
    document.body.appendChild(container);

    component.mount(container);
    const button = container.querySelector('button');

    button?.click();
    button?.click();
    button?.click();

    expect(component.clickCount).toBe(3);

    container.remove();
  });
});
