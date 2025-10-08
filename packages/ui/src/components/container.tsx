import React, { forwardRef } from "react";
import { motion, HTMLMotionProps, Variants } from "framer-motion";
import {
  pageVariants,
  cardVariants,
  containerVariants,
  listItemVariants,
  fadeVariants,
  scaleVariants,
  slideVariants,
  textVariants,
  buttonVariants,
  modalVariants,
  overlayVariants,
} from "@/animations/variants";

// Base container component
interface ContainerProps extends HTMLMotionProps<"div"> {
  variant?: "page" | "container" | "fade" | "scale";
  staggerChildren?: boolean;
  delay?: number;
}

export const Container = forwardRef<HTMLDivElement, ContainerProps>(
  (
    {
      variant = "fade",
      staggerChildren = false,
      delay = 0,
      children,
      ...props
    },
    ref,
  ) => {
    const getVariants = (): Variants => {
      switch (variant) {
        case "page":
          return pageVariants;
        case "container":
          return staggerChildren ? containerVariants : fadeVariants;
        case "scale":
          return scaleVariants;
        default:
          return fadeVariants;
      }
    };

    return (
      <motion.div
        ref={ref}
        variants={getVariants()}
        initial="hidden"
        animate="visible"
        exit="exit"
        style={{ "--delay": `${delay}s` } as React.CSSProperties}
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);

Container.displayName = "Container";

// Card component
interface CardProps extends HTMLMotionProps<"div"> {
  hover?: boolean;
  tap?: boolean;
  delay?: number;
}

export const Card = forwardRef<HTMLDivElement, CardProps>(
  ({ hover = true, tap = true, delay = 0, children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        variants={cardVariants}
        initial="hidden"
        animate="visible"
        whileHover={hover ? "hover" : undefined}
        whileTap={tap ? "tap" : undefined}
        style={{ "--delay": `${delay}s` } as React.CSSProperties}
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);

Card.displayName = "Card";

// List component
interface ListProps extends HTMLMotionProps<"div"> {
  staggerDelay?: number;
}

export const List = forwardRef<HTMLDivElement, ListProps>(
  ({ staggerDelay = 0.1, children, ...props }, ref) => {
    const staggerVariants = {
      hidden: { opacity: 0 },
      visible: {
        opacity: 1,
        transition: {
          staggerChildren: staggerDelay,
          delayChildren: 0.05,
        },
      },
    };

    return (
      <motion.div
        ref={ref}
        variants={staggerVariants}
        initial="hidden"
        animate="visible"
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);

List.displayName = "List";

// List item component
interface ListItemProps extends HTMLMotionProps<"div"> {
  index?: number;
}

export const ListItem = forwardRef<HTMLDivElement, ListItemProps>(
  ({ index, children, ...props }, ref) => {
    return (
      <motion.div ref={ref} variants={listItemVariants} key={index} {...props}>
        {children}
      </motion.div>
    );
  },
);

ListItem.displayName = "ListItem";

// Text component
interface TextProps extends HTMLMotionProps<"div"> {
  as?: "div" | "span" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6";
  delay?: number;
}

export const Text = forwardRef<HTMLElement, TextProps>(
  ({ as: Component = "div", delay = 0, children, ...props }, ref) => {
    const MotionComponent = motion[Component] as any;

    return (
      <MotionComponent
        ref={ref}
        variants={textVariants}
        initial="hidden"
        animate="visible"
        style={{ "--delay": `${delay}s` } as React.CSSProperties}
        {...props}
      >
        {children}
      </MotionComponent>
    );
  },
);

Text.displayName = "Text";

// Button component
interface ButtonProps extends HTMLMotionProps<"button"> {
  hover?: boolean;
  tap?: boolean;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ hover = true, tap = true, children, ...props }, ref) => {
    return (
      <motion.button
        ref={ref}
        variants={buttonVariants}
        initial="idle"
        whileHover={hover ? "hover" : undefined}
        whileTap={tap ? "tap" : undefined}
        {...props}
      >
        {children}
      </motion.button>
    );
  },
);

Button.displayName = "Button";

// Slide component
interface SlideProps extends HTMLMotionProps<"div"> {
  direction?: "left" | "right" | "up" | "down";
}

export const Slide = forwardRef<HTMLDivElement, SlideProps>(
  ({ direction = "left", children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        variants={slideVariants[direction]}
        initial="hidden"
        animate="visible"
        exit="exit"
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);

Slide.displayName = "Slide";

// Modal component
interface ModalProps extends HTMLMotionProps<"div"> {
  isOpen?: boolean;
}

export const Modal = forwardRef<HTMLDivElement, ModalProps>(
  ({ isOpen = true, children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        variants={modalVariants}
        initial="hidden"
        animate={isOpen ? "visible" : "hidden"}
        exit="exit"
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);

Modal.displayName = "Modal";

// Overlay component
export const Overlay = forwardRef<HTMLDivElement, HTMLMotionProps<"div">>(
  ({ children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        variants={overlayVariants}
        initial="hidden"
        animate="visible"
        exit="exit"
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);

Overlay.displayName = "Overlay";

// Page transition wrapper
interface PageTransitionProps extends HTMLMotionProps<"div"> {
  children: React.ReactNode;
}

export const PageTransition = forwardRef<HTMLDivElement, PageTransitionProps>(
  ({ children, ...props }, ref) => {
    return (
      <motion.div
        ref={ref}
        variants={pageVariants}
        initial="initial"
        animate="in"
        exit="out"
        {...props}
      >
        {children}
      </motion.div>
    );
  },
);

PageTransition.displayName = "PageTransition";

// Higher-order component for adding animations to any component
export function withAnimation<P extends object>(
  Component: React.ComponentType<P>,
  animationVariants: Variants = fadeVariants,
) {
  const AnimatedComponent = React.forwardRef<any, P>((props) => {
    return (
      <motion.div
        variants={animationVariants}
        initial="hidden"
        animate="visible"
        exit="exit"
      >
        <Component {...(props as P)} />
      </motion.div>
    );
  });

  AnimatedComponent.displayName = `WithAnimation(${Component.displayName || Component.name})`;

  return AnimatedComponent;
}
