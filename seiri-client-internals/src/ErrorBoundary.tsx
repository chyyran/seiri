import React from "react";

class ErrorBoundary extends React.Component<{},{hasError: boolean}> {
  constructor(props: {}) {
    super(props);
    this.state = { hasError: false };
  }

  public componentDidCatch(error: any) {
    // Display fallback UI
    // tslint:disable-next-line:no-console
    console.log(error);
    this.setState({ hasError: true });
  }

  public render() {
    if (this.state.hasError) {
      // You can render any custom fallback UI
      return <h1>Something went wrong.</h1>;
    }
    return this.props.children;
  }
}

export default ErrorBoundary