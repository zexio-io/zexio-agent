CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    domains JSON NOT NULL, -- List of domains ["example.com", "api.example.com"]
    encrypted_env BLOB,    -- Encrypted environment variables bundle
    webhook_secret TEXT,   -- Secret for verifying webhook headers
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_projects_id ON projects(id);
