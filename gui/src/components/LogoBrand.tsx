
interface LogoBrandProps {
    size?: "sm" | "md" | "lg";
}

export function LogoBrand({ size = "md" }: LogoBrandProps) {
    const dimensions = {
        sm: "w-6 h-6",
        md: "w-8 h-8",
        lg: "w-16 h-16"
    };

    return (
        <div className="flex flex-col items-center py-2">
            <img
                src="/logo.png"
                alt="Zexio"
                className={`${dimensions[size]} object-contain`}
            />
        </div>
    );
}
