import { Settings } from "lucide-react";
import { LogoBrand } from "./LogoBrand";

interface HeaderProps {
    onSettingsClick: () => void;
    onLogoClick?: () => void;
    mode: "cloud" | "standalone" | null;
}

export function Header({ onSettingsClick, onLogoClick, mode }: HeaderProps) {
    return (
        <div className="flex items-center justify-between px-6 py-4 border-b border-border">
            <button
                onClick={onLogoClick}
                className="flex items-center gap-4 hover:opacity-80 transition-opacity"
            >
                <LogoBrand size="sm" />
                {mode && (
                    <div className="px-2 py-0.5 text-[10px] font-medium bg-muted text-muted-foreground rounded-full border border-border">
                        {mode === "cloud" ? "☁️ Cloud" : "🏠 Standalone"}
                    </div>
                )}
            </button>

            <button
                onClick={onSettingsClick}
                className="p-2 hover:bg-border rounded-lg transition-colors"
            >
                <Settings className="w-5 h-5 text-muted-foreground" />
            </button>
        </div>
    );
}
