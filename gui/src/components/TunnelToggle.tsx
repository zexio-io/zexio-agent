interface TunnelToggleProps {
    isActive: boolean;
    onToggle: () => void;
}

export function TunnelToggle({ isActive, onToggle }: TunnelToggleProps) {
    return (
        <div className="flex flex-col items-center">
            <button
                onClick={onToggle}
                className="mb-8 relative"
            >
                <div className={`w-64 h-32 rounded-full transition-all duration-300 ${isActive
                        ? 'bg-gradient-to-r from-blue-500 to-cyan-500'
                        : 'bg-zinc-800'
                    }`}>
                    <div className={`absolute top-2 w-28 h-28 bg-white rounded-full transition-all duration-300 ${isActive ? 'right-2' : 'left-2'
                        }`} />
                </div>
            </button>

            <div className="text-center">
                <h3 className="text-3xl font-bold mb-2">
                    {isActive ? 'Connected' : 'Disconnected'}
                </h3>
                <p className="text-zinc-500">
                    {isActive
                        ? 'Your tunnel is active and running'
                        : 'Click the switch to start tunnel'}
                </p>
            </div>
        </div>
    );
}
