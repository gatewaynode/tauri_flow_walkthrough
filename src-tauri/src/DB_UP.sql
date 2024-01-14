CREATE TABLE IF NOT EXISTS nodes (
    id TEXT PRIMARY KEY,
    node_type TEXT NOT NULL,
    position TEXT NOT NULL,
    data TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS edges (
    id TEXT PRIMARY KEY,
    edge_type TEXT NOT NULL,
    source TEXT NOT NULL,
    target TEXT NOT NULL,
    animated BOOLEAN NOT NULL
);

INSERT INTO nodes (id, node_type, position, data) VALUES
    ('1', 'default', '{"x":100,"y":100}', '{"label":"Start"}'),
    ('2', 'default', '{"x":300,"y":100}', '{"label":"End"}');

INSERT INTO edges (id, edge_type, source, target, animated) VALUES
    ('e1-2', 'smoothstep', '1', '2', true);