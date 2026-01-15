interface ModeSelectorProps {
    onSelectCloud: () => void;
    onSelectStandalone: () => void;
}

export function ModeSelector({ onSelectCloud, onSelectStandalone }: ModeSelectorProps) {
    return (
        <div className="w-full max-w-2xl space-y-6">
            <div className="text-center mb-12">
                <h1 className="text-4xl font-bold mb-3">Welcome to Zexio Agent</h1>
                <p className="text-zinc-500">Choose your deployment mode to get started</p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* Cloud Mode */}
                <button
                    onClick={onSelectCloud}
                    className="group p-6 bg-zinc-900 border border-zinc-800 rounded-xl hover:border-blue-500 transition-all text-left"
                >
                    <div className="flex items-center gap-3 mb-4">
                        <div className="w-12 h-12 rounded-lg bg-blue-500/10 flex items-center justify-center text-2xl">
                            ☁️
                        </div>
                        <h3 className="text-xl font-bold">Zexio Cloud</h3>
                    </div>
                    <p className="text-sm text-zinc-500 mb-4">
                        Connect to Zexio Cloud for managed deployment with enterprise features
                    </p>
                    <ul className="space-y-2 text-sm text-zinc-400">
                        <li className="flex items-center gap-2">
                            <span className="text-green-400">✓</span>
                            Managed infrastructure
                        </li>
                        <li className="flex items-center gap-2">
                            <span className="text-green-400">✓</span>
                            Automatic updates
                        </li>
                        <li className="flex items-center gap-2">
                            <span className="text-green-400">✓</span>
                            Cloud monitoring
                        </li>
                    </ul>
                </button>

                {/* Standalone Mode */}
                <button
                    onClick={onSelectStandalone}
                    className="group p-6 bg-zinc-900 border border-zinc-800 rounded-xl hover:border-blue-500 transition-all text-left"
                >
                    <div className="flex items-center gap-3 mb-4">
                        <div className="w-12 h-12 rounded-lg bg-emerald-500/10 flex items-center justify-center text-2xl">
                            🏠
                        </div>
                        <h3 className="text-xl font-bold">Standalone</h3>
                    </div>
                    <p className="text-sm text-zinc-500 mb-4">
                        Self-hosted deployment with full control over your infrastructure
                    </p>
                    <ul className="space-y-2 text-sm text-zinc-400">
                        <li className="flex items-center gap-2">
                            <span className="text-green-400">✓</span>
                            Full control
                        </li>
                        <li className="flex items-center gap-2">
                            <span className="text-green-400">✓</span>
                            No cloud dependency
                        </li>
                        <li className="flex items-center gap-2">
                            <span className="text-green-400">✓</span>
                            Local deployment
                        </li>
                    </ul>
                </button>
            </div>
        </div>
    );
}
