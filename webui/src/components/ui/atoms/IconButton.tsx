import React from "react";

export interface IconButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  icon: React.ReactNode;
  label: string;
  variant?: "ghost" | "secondary" | "primary" | "danger";
  size?: "sm" | "md" | "lg";
}

const variantClasses = {
  ghost: "text-slate-400 hover:text-slate-100 hover:bg-slate-800/70 active:bg-slate-800",
  secondary:
    "bg-slate-800/80 text-slate-300 hover:text-white hover:bg-slate-700/80 border border-slate-700/60",
  primary: "bg-indigo-600 text-white hover:bg-indigo-500 active:bg-indigo-700 shadow-sm",
  danger: "text-slate-400 hover:text-rose-300 hover:bg-rose-950/40 active:bg-rose-950/60",
};

const sizeClasses = {
  sm: "p-1 rounded-md text-xs",
  md: "p-1.5 rounded-lg text-sm",
  lg: "p-2 rounded-xl text-base",
};

/**
 * Universal atomic accessible icon button with smooth states and tooltip label.
 */
export const IconButton: React.FC<IconButtonProps> = ({
  icon,
  label,
  variant = "ghost",
  size = "md",
  className = "",
  type = "button",
  ...props
}) => {
  return (
    <button
      type={type}
      aria-label={label}
      title={label}
      className={`inline-flex items-center justify-center transition-all focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 disabled:opacity-50 disabled:pointer-events-none ${variantClasses[variant]} ${sizeClasses[size]} ${className}`}
      {...props}
    >
      {icon}
    </button>
  );
};
