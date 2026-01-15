interface LogoBrandProps {
    size?: "sm" | "md" | "lg";
}

export function LogoBrand({ size = "lg" }: LogoBrandProps) {
    const sizes = {
        sm: "w-12 h-12",
        md: "w-16 h-16",
        lg: "w-24 h-24"
    };

    return (
        <div className="flex flex-col items-center">
            <img
                src="/logo.png"
                alt="Zexio"
                className={`${sizes[size]} mb-4`}
            />
            <p className="text-center text-sm text-zinc-500">Agent</p>
        </div>
    );
}
