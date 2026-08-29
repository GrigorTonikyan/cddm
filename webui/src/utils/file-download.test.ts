import { describe, it, expect, vi } from "vite-plus/test";
import { downloadTextFile } from "./file-download";

describe("downloadTextFile", () => {
  it("creates an anchor and triggers click for download", () => {
    const createObjectURLMock = vi.fn().mockReturnValue("blob:http://localhost/1234");
    const revokeObjectURLMock = vi.fn();
    globalThis.URL.createObjectURL = createObjectURLMock;
    globalThis.URL.revokeObjectURL = revokeObjectURLMock;

    const clickMock = vi.fn();
    const appendChildMock = vi.spyOn(document.body, "appendChild").mockImplementation((node) => {
      (node as HTMLAnchorElement).click = clickMock;
      return node;
    });
    const removeChildMock = vi
      .spyOn(document.body, "removeChild")
      .mockImplementation((node) => node);

    downloadTextFile("diff content", "test.patch", "text/x-diff;charset=utf-8");

    expect(createObjectURLMock).toHaveBeenCalledTimes(1);
    expect(appendChildMock).toHaveBeenCalledTimes(1);
    expect(clickMock).toHaveBeenCalledTimes(1);
    expect(removeChildMock).toHaveBeenCalledTimes(1);
    expect(revokeObjectURLMock).toHaveBeenCalledWith("blob:http://localhost/1234");

    appendChildMock.mockRestore();
    removeChildMock.mockRestore();
  });
});
