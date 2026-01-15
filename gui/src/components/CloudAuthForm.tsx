import { useState } from "react";

interface CloudAuthFormProps {
    onSubmit: (token: string, nodeId: string) => void;
    onBack: () => void;
}

export function CloudAuthForm({ onSubmit, onBack }: CloudAuthFormProps) {
    const [token, setToken] = useState("");
    const [nodeId, setWorkerId] = useState("");

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (token.trim() && nodeId.trim()) {
            onSubmit(token, nodeId);
        }
    };

    return (
        <div className="w-full max-w-md space-y-6">
            <div className="text-center mb-8">
                <h2 className="text-2xl font-bold mb-2">Connect to Zexio Cloud</h2>
                <p className="text-sm text-muted-foreground">
                    Enter your credentials to connect this agent to Zexio Cloud
                </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                    <label className="block text-sm text-muted-foreground mb-2">
                        Zexio Token
                    </label>
                    <textarea
                        value={token}
                        onChange={(e) => setToken(e.target.value)}
                        placeholder="Paste your Zexio authentication token here..."
                        rows={4}
                        className="w-full px-4 py-3 bg-card border border-border rounded-lg text-white placeholder-muted-foreground focus:outline-none focus:border-primary font-mono text-xs resize-none"
                        required
                    />
                    <p className="text-xs text-muted-foreground mt-1">
                        Get your token from Zexio Cloud dashboard
                    </p>
                </div>

                <div>
                    <label className="block text-sm text-muted-foreground mb-2">
                        Node ID
                    </label>
                    <input
                        type="text"
                        value={nodeId}
                        onChange={(e) => setWorkerId(e.target.value)}
                        placeholder="node_xxxxxxxxxxxxx"
                        className="w-full px-4 py-3 bg-card border border-border rounded-lg text-white placeholder-muted-foreground focus:outline-none focus:border-primary"
                        required
                    />
                    <p className="text-xs text-muted-foreground mt-1">
                        Unique identifier for this edge or managed node
                    </p>
                </div>

                <div className="flex gap-3 pt-2">
                    <button
                        type="button"
                        onClick={onBack}
                        className="flex-1 py-3 bg-border hover:bg-muted rounded-lg font-medium transition-colors"
                    >
                        Back
                    </button>
                    <button
                        type="submit"
                        className="flex-1 py-3 bg-primary hover:bg-primary rounded-lg font-medium transition-colors"
                    >
                        Connect
                    </button>
                </div>
            </form>
        </div>
    );
}
