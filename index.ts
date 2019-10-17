import {
  create_tree,
  insert_to_tree,
  nearest_in_tree,
  remove_from_tree
} from "./pkg/rstar_wasm";

export class RTree {
  private readonly uid = Math.random().toString();

  constructor() {
    create_tree(this.uid);
  }

  insert(point: [number, number]) {
    insert_to_tree(this.uid, this.wasmPoint(point));
  }

  nearest(point: [number, number]) {
    const wasmPoint = nearest_in_tree(this.uid, this.wasmPoint(point));
    if (wasmPoint.length) {
      return [wasmPoint[0], wasmPoint[1]];
    }
    return [];
  }

  destroy() {
    remove_from_tree(this.uid);
  }

  private wasmPoint(point: [number, number]) {
    let wasmPoint = new Float64Array(2);
    wasmPoint[0] = point[0];
    wasmPoint[1] = point[1];
    return wasmPoint;
  }
}
