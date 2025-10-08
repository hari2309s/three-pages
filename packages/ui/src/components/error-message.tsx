"use client";

import { AlertCircle } from "lucide-react";
import { cn } from "../lib/utils";

interface ErrorMessageProps {
  message: string;
  className?: string;
}

export const ErrorMessage = ({ message, className }: ErrorMessageProps) => {
  return (
    <div
      className={cn(
        "flex items-center gap-2 rounded-lg border border-destructive/50 bg-destructive/10 p-4 text-destructive",
        className,
      )}
    >
      <AlertCircle className="h-5 w-5 flex-shrink-0" />
      <p className="text-sm">{message}</p>
    </div>
  );
};
