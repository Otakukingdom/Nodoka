/**
 * Base component class for vanilla TypeScript UI components
 * Provides lifecycle management and DOM element references
 */

export abstract class Component {
  protected element: HTMLElement | null = null;

  /**
   * Render the component and return the DOM element
   */
  abstract render(): HTMLElement;

  /**
   * Called after the element is added to the DOM
   */
  protected afterMount(): void {
    // Override in subclasses
  }

  /**
   * Called before the element is removed from the DOM
   */
  protected beforeUnmount(): void {
    // Override in subclasses
  }

  /**
   * Mount the component to a parent element
   */
  mount(parent: HTMLElement): this {
    this.element = this.render();
    parent.appendChild(this.element);
    this.afterMount();
    return this;
  }

  /**
   * Unmount the component from the DOM
   */
  unmount(): void {
    if (this.element) {
      this.beforeUnmount();
      this.element.remove();
      this.element = null;
    }
  }

  /**
   * Update the component (re-render)
   */
  protected update(): void {
    const parent = this.element?.parentNode;
    if (this.element !== null && parent !== null && parent !== undefined) {
      this.beforeUnmount();
      this.element.remove();
      this.element = this.render();
      parent.appendChild(this.element);
      this.afterMount();
    }
  }
}
