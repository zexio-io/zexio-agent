import { useState } from "react";

interface CloudAuthFormProps {
    onSubmit: (orgId: string, token: string, nodeId: string) => void;
    onBack: () => void;
}

export function CloudAuthForm({ onSubmit, onBack }: CloudAuthFormProps) {
    const [orgId, setOrgId] = useState("");
    const [token, setToken] = useState("");
    const [nodeId, setNodeId] = useState("");

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (orgId.trim() && token.trim() && nodeId.trim()) {
            onSubmit(orgId, token, nodeId);
        }
    };

    return (
        <div className="w-full max-w-md space-y-6">
            <div className="text-center mb-8">
                <h2 className="text-2xl font-bold mb-2">Connect to Zexio Cloud</h2>
                <p className="text-sm text-zinc-500">
                    Enter your organization credentials to connect
                </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-4">
                <div>
                    <label className="block text-sm text-zinc-400 mb-2">
                        Organization ID
                    </label>
                    <input
                        type="text"
                        value={orgId}
                        onChange={(e) => setOrgId(e.target.value)}
                        placeholder="org_xxxxxxxxxxxxx"
                        className="w-full px-4 py-3 bg-zinc-900 border border-zinc-800 rounded-lg text-white placeholder-zinc-600 focus:outline-none focus:border-blue-500"
                        required
                    />
                </div>

                <div>
                    <label className="block text-sm text-zinc-400 mb-2">
                        Node ID
                    </label>
                    <input
                        type="text"
                        value={nodeId}
                        onChange={(e) => setNodeId(e.target.value)}
                        placeholder="node_xxxxxxxxxxxxx"
                        className="w-full px-4 py-3 bg-zinc-900 border border-zinc-800 rounded-lg text-white placeholder-zinc-600 focus:outline-none focus:border-blue-500"
                        required
                    />
                </div>

                <div>
                    <label className="block text-sm text-zinc-400 mb-2">
                        Authentication Token
                    </label>
                    <textarea
                        value={token}
                        onChange={(e) => setToken(e.target.value)}
                        placeholder="Paste your authentication token here..."
                        rows={4}
                        className="w-full px-4 py-3 bg-zinc-900 border border-zinc-800 rounded-lg text-white placeholder-zinc-600 focus:outline-none focus:border-blue-500 font-mono text-xs"
                        required
                    />
                </div>

                <div className="flex gap-3">
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
