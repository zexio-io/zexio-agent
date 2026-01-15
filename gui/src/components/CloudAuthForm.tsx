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
                <p className="text-sm text-zinc-500">
                    Enter your credentials to connect this agent to Zexio Cloud
                </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                    <label className="block text-sm text-zinc-400 mb-2">
                        Zexio Token
                    </label>
                    <textarea
                        value={token}
                        onChange={(e) => setToken(e.target.value)}
                        placeholder="Paste your Zexio authentication token here..."
                        rows={4}
                        className="w-full px-4 py-3 bg-zinc-900 border border-zinc-800 rounded-lg text-white placeholder-zinc-600 focus:outline-none focus:border-blue-500 font-mono text-xs resize-none"
                        required
                    />
                    <p className="text-xs text-zinc-600 mt-1">
                        Get your token from Zexio Cloud dashboard
                    </p>
                </div>

                <div>
                    <label className="block text-sm text-zinc-400 mb-2">
                        Node ID
                    </label>
                    <input
                        type="text"
                        value={nodeId}
                        onChange={(e) => setWorkerId(e.target.value)}
                        placeholder="node_xxxxxxxxxxxxx"
                        className="w-full px-4 py-3 bg-zinc-900 border border-zinc-800 rounded-lg text-white placeholder-zinc-600 focus:outline-none focus:border-blue-500"
                        required
                    />
                    <p className="text-xs text-zinc-600 mt-1">
                        Unique identifier for this edge or managed node
                    </p>
                </div>

                <div className="flex gap-3 pt-2">
                    <button
                        type="button"
                        onClick={onBack}
                        className="flex-1 py-3 bg-zinc-800 hover:bg-zinc-700 rounded-lg font-medium transition-colors"
                    >
                        Back
                    </button>
                    <button
                        type="submit"
                        className="flex-1 py-3 bg-blue-600 hover:bg-blue-500 rounded-lg font-medium transition-colors"
                    >
                        Connect
                    </button>
                </div>
            </form>
        </div>
    );
}
