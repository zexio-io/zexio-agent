interface StandaloneConfigProps {
    onSubmit: (apiPort: number, meshPort: number) => void;
    onBack: () => void;
}

export function StandaloneConfig({ onSubmit, onBack }: StandaloneConfigProps) {
    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        onSubmit(8080, 8081);
    };

    return (
        <div className="w-full max-w-md space-y-6">
            <div className="text-center mb-8">
                <h2 className="text-2xl font-bold mb-2">Standalone Mode</h2>
                <p className="text-sm text-zinc-500">
                    Agent will run in self-hosted mode
                </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-4">
                <div className="bg-zinc-900 border border-zinc-800 rounded-lg p-4 space-y-3">
                    <div className="flex items-center justify-between">
                        <span className="text-sm text-zinc-400">API Port</span>
                        <span className="text-sm font-medium">8080</span>
                    </div>
                    <div className="flex items-center justify-between">
                        <span className="text-sm text-zinc-400">Mesh Port</span>
                        <span className="text-sm font-medium">8081</span>
                    </div>
                </div>

                <div className="bg-blue-500/10 border border-blue-500/20 rounded-lg p-4">
                    <p className="text-sm text-blue-400">
                        ℹ️ No cloud connection required. All features will run locally.
                    </p>
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
                        Continue
                    </button>
                </div>
            </form>
        </div>
    );
}
