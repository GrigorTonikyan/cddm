import { describe, it, expect } from "vite-plus/test";
import { render, screen } from "@testing-library/react";
import { lazyModal } from "./lazy-modal";
import React, { Suspense } from "react";

describe("lazyModal helper", () => {
  it("should create and render a React.lazy component from named export", async () => {
    const DummyComponent: React.FC = () => <div data-testid="dummy">Modal Content</div>;
    const fakeModule = async () => ({ MyModal: DummyComponent });

    const LazyComp = lazyModal<React.FC>(fakeModule, "MyModal");
    expect(LazyComp).toBeDefined();

    render(
      <Suspense fallback={<div>Loading...</div>}>
        <LazyComp />
      </Suspense>,
    );

    expect(await screen.findByTestId("dummy")).toBeDefined();
    expect(screen.getByText("Modal Content")).toBeDefined();
  });
});
