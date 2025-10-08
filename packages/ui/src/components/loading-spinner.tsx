"use client";

import { motion } from "framer-motion";
import { cn } from "../lib/utils";
import { spinnerVariants } from "../animations/variants";

interface LoadingSpinnerProps {
  className?: string;
  size?: "sm" | "md" | "lg";
}

export const LoadingSpinner = ({
  className,
  size = "md",
}: LoadingSpinnerProps) => {
  return (
    <div className={cn("flex items-center justify-center", className)}>
      <motion.div
        variants={spinnerVariants}
        animate="animate"
        className={cn("rounded-full border-b-2 border-primary", {
          "h-4 w-4": size === "sm",
          "h-8 w-8": size === "md",
          "h-12 w-12": size === "lg",
        })}
      />
    </div>
  );
};
