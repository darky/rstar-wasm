import { RTree } from "./index";

describe("RTree", () => {
  it("Nearest", () => {
    const tree = new RTree();
    tree.insert([1, 1]);
    tree.insert([2, 2]);
    const nearest = tree.nearest([2.2, 2.2]);
    expect(nearest).toEqual([2, 2]);
    tree.destroy();
  });

  it("Not found point", () => {
    const tree = new RTree();
    const nearest = tree.nearest([2.2, 2.2]);
    expect(nearest).toEqual([]);
    tree.destroy();
  });
});
