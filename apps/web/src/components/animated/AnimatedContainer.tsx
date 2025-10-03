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
} from "../../utils/animations";

// Base animated container component
interface AnimatedContainerProps extends HTMLMotionProps<"div"> {
  variant?: "page" | "container" | "fade" | "scale";
  staggerChildren?: boolean;
  delay?: number;
}

export const AnimatedContainer = forwardRef<
  HTMLDivElement,
  AnimatedContainerProps
>(
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

AnimatedContainer.displayName = "AnimatedContainer";

// Animated card component
interface AnimatedCardProps extends HTMLMotionProps<"div"> {
  hover?: boolean;
  tap?: boolean;
  delay?: number;
}

export const AnimatedCard = forwardRef<HTMLDivElement, AnimatedCardProps>(
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

AnimatedCard.displayName = "AnimatedCard";

// Animated list component
interface AnimatedListProps extends HTMLMotionProps<"div"> {
  staggerDelay?: number;
}

export const AnimatedList = forwardRef<HTMLDivElement, AnimatedListProps>(
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

AnimatedList.displayName = "AnimatedList";

// Animated list item component
interface AnimatedListItemProps extends HTMLMotionProps<"div"> {
  index?: number;
}

export const AnimatedListItem = forwardRef<
  HTMLDivElement,
  AnimatedListItemProps
>(({ index = 0, children, ...props }, ref) => {
  return (
    <motion.div ref={ref} variants={listItemVariants} {...props}>
      {children}
    </motion.div>
  );
});

AnimatedListItem.displayName = "AnimatedListItem";

// Animated text component
interface AnimatedTextProps extends HTMLMotionProps<"div"> {
  as?: "div" | "span" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6";
  delay?: number;
}

export const AnimatedText = forwardRef<HTMLElement, AnimatedTextProps>(
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

AnimatedText.displayName = "AnimatedText";

// Animated button component
interface AnimatedButtonProps extends HTMLMotionProps<"button"> {
  hover?: boolean;
  tap?: boolean;
}

export const AnimatedButton = forwardRef<
  HTMLButtonElement,
  AnimatedButtonProps
>(({ hover = true, tap = true, children, ...props }, ref) => {
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
});

AnimatedButton.displayName = "AnimatedButton";

// Animated slide component
interface AnimatedSlideProps extends HTMLMotionProps<"div"> {
  direction?: "left" | "right" | "up" | "down";
}

export const AnimatedSlide = forwardRef<HTMLDivElement, AnimatedSlideProps>(
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

AnimatedSlide.displayName = "AnimatedSlide";

// Animated modal component
interface AnimatedModalProps extends HTMLMotionProps<"div"> {
  isOpen?: boolean;
}

export const AnimatedModal = forwardRef<HTMLDivElement, AnimatedModalProps>(
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

AnimatedModal.displayName = "AnimatedModal";

// Animated overlay component
export const AnimatedOverlay = forwardRef<
  HTMLDivElement,
  HTMLMotionProps<"div">
>(({ children, ...props }, ref) => {
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
});

AnimatedOverlay.displayName = "AnimatedOverlay";

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
