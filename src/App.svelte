<script>
  import { writable } from "svelte/store";
  import { SvelteFlow, Background, BackgroundVariant } from "@xyflow/svelte";
  import "@xyflow/svelte/dist/style.css";
  import "./node-styles.css";
  import { invoke } from "@tauri-apps/api/core";

  const nodes = writable([]);
  const edges = writable([]);
  const snapGrid = [25, 25];

  async function _get_nodes() {
    let _all_nodes = await invoke("get_nodes");
    console.log(_all_nodes);
    // Our promise to remap the object keys after await returns everything
    // so we can translate the "node_type" key to "type"
    Promise.allSettled(
      _all_nodes.map((item) => {
        delete Object.assign(item, { ["type"]: item["node_type"] })[
          "node_type"
        ];
      }),
    );
    console.log(_all_nodes);
    // Loop through the list of objects and add them to the nodes variable
    for (let _node of _all_nodes) {
      console.log(_node);
      nodes.update((n) => [
        ...n,
        {
          id: _node.id,
          type: _node.type,
          position: JSON.parse(_node.position),
          data: JSON.parse(_node.data),
        },
      ]);
    }
  }
  _get_nodes();

  async function _get_edges() {
    let _all_edges = await invoke("get_edges");
    console.log(_all_edges);
    // Our promise to remap the object keys after await returns everything
    // so we can translate the "edge_type" key to "type"
    Promise.allSettled(
      _all_edges.map((item) => {
        delete Object.assign(item, { ["type"]: item["edge_type"] })[
          "edge_type"
        ];
      }),
    );
    console.log(_all_edges);
    // return _all_edges;
    // Loop through the list of objects and add them to the edges variable
    for (let _edge of _all_edges) {
      console.log(_edge);
      edges.update((e) => [
        ...e,
        {
          id: _edge.id,
          source: _edge.source,
          target: _edge.target,
          type: _edge.type,
          animated: true,
        },
      ]);
    }
  }
  _get_edges();
</script>

<main class="container">
  <div style:height="500px">
    <SvelteFlow
      {nodes}
      {edges}
      {snapGrid}
      fitView
      on:nodeclick={(event) => console.log("on node click", event.detail.node)}
    >
      <Background variant={BackgroundVariant.Dots} />
    </SvelteFlow>
  </div>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
