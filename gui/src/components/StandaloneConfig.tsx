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
                <p className="text-sm text-muted-foreground">
                    Agent will run in self-hosted mode
                </p>
            </div>

            <form onSubmit={handleSubmit} className="space-y-4">
                <div className="bg-card border border-border rounded-lg p-4 space-y-3">
                    <div className="flex items-center justify-between">
                        <span className="text-sm text-muted-foreground">API Port</span>
                        <span className="text-sm font-medium">8080</span>
                    </div>
                    <div className="flex items-center justify-between">
                        <span className="text-sm text-muted-foreground">Mesh Port</span>
                        <span className="text-sm font-medium">8081</span>
                    </div>
                </div>

                <div className="bg-primary/10 border border-primary/20 rounded-lg p-4">
                    <p className="text-sm text-primary">
                        ℹ️ No cloud connection required. All features will run locally.
                    </p>
                </div>

                <div className="flex gap-3">
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
                        Continue
                    </button>
                </div>
            </form>
        </div>
    );
}
