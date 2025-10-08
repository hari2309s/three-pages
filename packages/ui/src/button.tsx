"use client";

import * as React from "react";
import { motion } from "framer-motion";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "./lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center whitespace-nowrap text-sm font-medium transition-all duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default:
          "bg-black text-white rounded-lg shadow-md border border-slate-700",
        destructive:
          "bg-red-600 text-white rounded-lg shadow-md border border-red-500",
        outline:
          "border-2 border-slate-300 bg-transparent text-slate-900 rounded-lg shadow-sm",
        secondary:
          "bg-slate-600 text-white rounded-lg shadow-md border border-slate-500",
        ghost: "text-slate-900 rounded-lg",
        link: "text-blue-600 underline-offset-4 hover:underline",
      },
      size: {
        default: "h-10 px-6 py-2 min-w-[80px]",
        sm: "h-8 px-4 py-1 text-xs min-w-[60px]",
        lg: "h-12 px-8 py-3 text-base min-w-[100px]",
        icon: "h-10 w-10 p-2",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
  animated?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (
    { className, variant, size, asChild = false, animated = true, ...props },
    ref,
  ) => {
    const buttonClassName = cn(buttonVariants({ variant, size, className }));

    if (asChild) {
      return <Slot className={buttonClassName} ref={ref} {...props} />;
    }

    if (animated) {
      const {
        onAnimationStart,
        onAnimationEnd,
        onDragStart,
        onDrag,
        onDragEnd,
        ...buttonProps
      } = props;
      return (
        <motion.button
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          transition={{ type: "spring", stiffness: 300, damping: 20 }}
          className={buttonClassName}
          ref={ref}
          {...buttonProps}
        />
      );
    }

    return <button className={buttonClassName} ref={ref} {...props} />;
  },
);

Button.displayName = "Button";

export { Button, buttonVariants };
